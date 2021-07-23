use crate::db::ipv4_insert;
use crate::ed25519::seed;
use crate::util::addr_to_bytes::ToBytes;
use crate::util::now;
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
use twox_hash::xxh3::hash128;

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
  let cmd_key_public_bytes = [&[CMD::KEY], &public.as_bytes()[..30]].concat();
  println!("public {:?}", cmd_key_public_bytes);

  loop {
    match socket.recv_from(&mut input).await {
      Err(err) => error!("{:?}", err),
      Ok((n, src)) => {
        macro_rules! reply {
          ($val:expr) => {
            send_to!($val, src);
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
                  let key = &input[1..];
                  if key.len() == 30 {
                    reply!([
                      &[CMD::Q],
                      &hash128(&[&src.to_bytes(), key].concat()).to_le_bytes()[..]
                    ]
                    .concat());
                  }
                }
                CMD::Q => {
                  let key = src.to_bytes();
                  if let Some(_) = connecting.expiration(&key).await {
                    connecting.remove(&key).await;
                    ipv4_insert(key)?;
                    unsafe { CONNECTED_TIME = now::sec() };

                    let hash = &input[1..];
                    println!("Hash {:?}", hash);
                  }
                }
                CMD::A => {}
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
