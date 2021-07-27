use crate::db::ipv4_insert;
use crate::ed25519::seed;
use crate::util::addr_to_bytes::ToBytes;
use crate::util::{leading_zero, now};
use crate::var::cmd::CMD;
use crate::var::msl::MSL;
use anyhow::Result;
use async_std::net::UdpSocket;
use bytes::BytesMut;
use ed25519_dalek_blake3::{PublicKey, SecretKey};
use log::{error, info};
use retainer::Cache;
use static_init::dynamic;
use std::net::SocketAddr::V4;
use twox_hash::xxh3::{hash128, hash64};

//use crate::encode;
//use crate::file::test;
/*
  encode::u64();
  match test() {
    Ok(_) => {
      info!("test ok");
    }
    Err(err) => {
      error!("{:?}", err);
    }
  }
*/

#[dynamic]
pub static MTU: usize = {
  let mut mtu = config_get!(mtu, { 1472.to_string() }).parse().unwrap();
  if mtu < 548 {
    mtu = 548
  }
  mtu
};

pub static mut CONNECTED_TIME: u64 = 0;

pub async fn recv_from(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) -> Result<()> {
  macro_rules! send_to {
    ($val:expr, $addr:expr) => {
      Await!(socket.send_to(&$val, $addr));
    };
  }

  let mut input = BytesMut::new();
  input.resize(*MTU, 0);

  let secret = SecretKey::from_bytes(&seed()).unwrap();
  let public: PublicKey = (&secret).into();
  let public_bytes = &public.as_bytes()[..30];
  let cmd_key_public_bytes = [&[CMD::KEY], public_bytes].concat();

  loop {
    match socket.recv_from(&mut input).await {
      Err(err) => error!("{:?}", err),
      Ok((n, src)) => {
        macro_rules! reply {
          ($val:expr) => {
            send_to!($val, src);
          };
          ($cmd:expr, $val:expr) => {
            reply!([&[$cmd], &$val[..]].concat());
          };
        }

        match src {
          V4(src) => {
            if n > 0 {
              match input[0] {
                CMD::PING => reply!([CMD::PONG]),
                CMD::PONG => {
                  if let Some(instant) = connecting.expiration(&src.to_bytes()).await {
                    info!(
                      "ip pong {} connecting elapsed {:?}",
                      src,
                      (instant - *MSL).elapsed()
                    );
                    reply!(cmd_key_public_bytes);
                  }
                }
                CMD::KEY => {
                  if n == 31 {
                    reply!(
                      CMD::Q,
                      hash128(&[&src.to_bytes(), &input[1..31], public_bytes].concat())
                        .to_le_bytes()
                    );
                  }
                }
                CMD::Q => {
                  let key = src.to_bytes();
                  if let Some(_) = connecting.expiration(&key).await {
                    connecting.remove(&key).await;
                    ipv4_insert(key)?;
                    unsafe { CONNECTED_TIME = now::sec() };

                    reply!([
                      &[CMD::A],
                      &public_bytes[..],
                      &leading_zero::find(21, &input[1..n], hash64)
                    ]
                    .concat());
                  }
                }
                CMD::A => {
                  let key = &input[1..31];
                  let token = &input[31..n];
                  info!("key: {:?} token: {:?}", key, token);
                  info!(
                    "leading zero: {}",
                    hash64(
                      &[
                        &hash128(&[&src.to_bytes(), key, public_bytes].concat()).to_le_bytes()[..],
                        &token
                      ]
                      .concat()
                    )
                    .leading_zeros()
                  );
                }
                _ => {
                  info!("{}  > {} : {:?}", src, input[0], &input[1..]);
                }
              }
            }
          }
          _ => {}
        }
      }
    }
  }
}
