use crate::db::ipv4_insert;
use crate::ed25519::seed;
use crate::util::addr_to_bytes::ToBytes;
use crate::util::{leading_zero, now};
use crate::var::cmd::CMD;
use crate::var::msl::MSL;
use anyhow::Result;
use async_std::net::UdpSocket;
use bytes::BytesMut;
use ed25519_dalek_blake3::{ExpandedSecretKey, PublicKey, SecretKey, Signature};
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
pub const QA_LEADING_ZERO: u32 = 16;
pub const PUBLIC_KEY_LENGTH: usize = 30;
pub const PUBLIC_KEY_LENGTH_1: usize = PUBLIC_KEY_LENGTH + 1;

pub async fn recv_from(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) -> Result<()> {
  macro_rules! send_to {
    ($val:expr, $addr:expr) => {
      Await!(socket.send_to(&$val, $addr));
    };
  }

  let mut input = BytesMut::new();
  input.resize(*MTU, 0);

  let secret = SecretKey::from_bytes(&seed()).unwrap();
  let signer: ExpandedSecretKey = (&secret).into();
  let public: PublicKey = (&secret).into();
  let public_bytes = &public.as_bytes()[..PUBLIC_KEY_LENGTH];
  let cmd_send_key = [&[CMD::SEND_KEY], public_bytes].concat();
  let cmd_public_key = [&[CMD::PUBLIC_KEY], public_bytes].concat();

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
                    reply!(cmd_send_key);
                  }
                }
                CMD::SEND_KEY => {
                  if n == PUBLIC_KEY_LENGTH_1 {
                    let key = &input[1..PUBLIC_KEY_LENGTH_1];
                    if key != public_bytes {
                      reply!(
                        CMD::Q,
                        hash128(&[&src.to_bytes(), key, public_bytes].concat()).to_le_bytes()
                      );
                    }
                  }
                }
                CMD::Q => {
                  if let Some(_) = connecting.expiration(&src.to_bytes()).await {
                    let q = &input[1..n];
                    let token = &leading_zero::find(QA_LEADING_ZERO, q, hash64);
                    let sign = signer.sign(q, &public).to_bytes();

                    reply!([&[CMD::A], &public_bytes[..], &sign[..], token].concat());
                  }
                }
                CMD::A => {
                  if n >= PUBLIC_KEY_LENGTH_1 + 64 {
                    let key = &input[1..PUBLIC_KEY_LENGTH_1];
                    let sign = Signature::new(
                      input[PUBLIC_KEY_LENGTH_1..PUBLIC_KEY_LENGTH_1 + 64]
                        .try_into()
                        .unwrap(),
                    );
                    let token = &input[PUBLIC_KEY_LENGTH_1 + 64..n];
                    let hash =
                      &hash128(&[&src.to_bytes(), key, public_bytes].concat()).to_le_bytes()[..];

                    if hash64(&[hash, &token].concat()).leading_zeros() >= QA_LEADING_ZERO {
                      let pk = PublicKey::from_bytes(&[key, &[0, 0]].concat()).unwrap();
                      if let Ok(_) = pk.verify_strict(hash, &sign) {
                        // 设置id
                        // 生成秘钥
                        // 响应加密后的id
                        let id = 0u32.to_le_bytes();
                        reply!(cmd_public_key);
                      }
                    }
                  };
                }
                CMD::PUBLIC_KEY => {
                  let src_bytes = src.to_bytes();
                  if let Some(_) = connecting.expiration(&src_bytes).await {
                    connecting.remove(&src_bytes).await;
                    ipv4_insert(src_bytes)?;
                    unsafe { CONNECTED_TIME = now::sec() };
                    info!("public key {:?}", &input[..PUBLIC_KEY_LENGTH_1]);
                  }
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
