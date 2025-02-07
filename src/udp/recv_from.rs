use crate::kad::KAD;
use crate::seed::seed;
use crate::udp::state::SPEED;
use crate::util::{addr_to_bytes::ToBytes, leading_zero, now};
use crate::var::{
  cmd::CMD,
  duration::{HEARTBEAT_TIMEOUT, MSL},
  len::PUBLIC_KEY_LENGTH,
};
use anyhow::Result;
use async_std::net::UdpSocket;
use bytes::BytesMut;
use ed25519_dalek_blake3::{ExpandedSecretKey, PublicKey, SecretKey, Signature};
use log::{error, info};
use retainer::Cache;
use static_init::dynamic;
use std::hash::Hasher;
use std::net::SocketAddr::{V4, V6};
use std::net::SocketAddrV4;
use twox_hash::{
  xxh3::{hash128, hash64},
  XxHash32,
};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use xxblake3::{decrypt, encrypt};

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

fn public_key_from_bytes(bytes: &[u8]) -> PublicKey {
  PublicKey::from_bytes(&[bytes, &[0, 0]].concat()).unwrap()
}

pub const PUBLIC_KEY_LENGTH_1: usize = PUBLIC_KEY_LENGTH + 1;
pub const PUBLIC_KEY_LENGTH_13: usize = PUBLIC_KEY_LENGTH + 13;

pub async fn recv_from(
  socket: &UdpSocket,
  connecting: &Cache<SocketAddrV4, u64>,
  /*
  connected: &Cache<u32, [u8; 32]>,
  */
) -> Result<()> {
  macro_rules! send_to {
    ($val:expr, $addr:expr) => {
      Await!(socket.send_to(&$val, $addr))
    };
  }

  let mut input = BytesMut::new();
  input.resize(*MTU, 0);

  let secret = SecretKey::from_bytes(&seed()).unwrap();
  //let signer: ExpandedSecretKey = (&secret).into();
  let public: PublicKey = (&secret).into();
  let x25519_secret: StaticSecret = secret.into();
  let public_bytes = &public.as_bytes()[..PUBLIC_KEY_LENGTH];
  let cmd_ping_key = [&[CMD::PING], public_bytes].concat();
  let cmd_pong_key = [&[CMD::PONG], public_bytes].concat();

  loop {
    match socket.recv_from(&mut input).await {
      Err(err) => error!("{:?}", err),
      Ok((n, src)) => {
        macro_rules! reply {
          ($val:expr) => {
            send_to!($val, src)
          };
          ($cmd:expr, $val:expr) => {
            reply!([&[$cmd], &$val[..]].concat());
          };
        }

        if n > 0 {
          match src {
            V4(src) => match input[0] {
              CMD::PING => {
                if n == 1 {
                  reply!([CMD::PONG])
                } else if n == PUBLIC_KEY_LENGTH_1 {
                  let key = &input[1..];
                  if key != public_bytes {
                    reply!(cmd_pong_key)
                  }
                }
              }
              CMD::PONG => {
                if n == 1 {
                  if connecting.renew(&src, *MSL).await {
                    reply!(cmd_ping_key)
                  }
                } else if n == PUBLIC_KEY_LENGTH_1 {
                  if let Some(_) = connecting.expiration(&src).await {
                    let key = &input[1..];
                    KAD.write().add(key.try_into().unwrap(), src);
                    connecting.remove(&src).await;
                  }
                }
              }
              _ => {}
            },
            V6(_) => {}
          }
          unsafe { SPEED.incr(n) };
        }
      }
    }
  }
}

/*

              match input[0] {
                CMD::PING => reply!([CMD::PONG]),
                CMD::PONG => {
                  if connecting.renew(&src.to_bytes(), *MSL).await {
                    reply!(cmd_key);
                  }
                }
                CMD::KEY => match n {
                  PUBLIC_KEY_LENGTH_1 => {
                    let key = &input[1..PUBLIC_KEY_LENGTH_1];
                    if key != public_bytes {
                      reply!(
                        CMD::Q,
                        hash128(&[&src.to_bytes(), key, public_bytes].concat()).to_le_bytes()
                      );
                    }
                  }
                  PUBLIC_KEY_LENGTH_13 => {
                    let src_bytes = src.to_bytes();
                    let key = &input[1..PUBLIC_KEY_LENGTH_1];
                    let pk = public_key_from_bytes(key);
                    let xpk: X25519PublicKey = pk.into();
                    let xsecret = x25519_secret.diffie_hellman(&xpk);
                    let xsecret = xsecret.as_bytes();
                    if let Some(connect_id) = decrypt(
                      xsecret,
                      &input[PUBLIC_KEY_LENGTH_1..PUBLIC_KEY_LENGTH_1 + 12],
                    ) {
                      let connect_id = (*connect_id).try_into().unwrap();
                      let connect_id = u32::from_le_bytes(connect_id);

                      let mut id = connect_id;
                      loop {
                        match connected.get(&id).await {
                          None => break,
                          Some(val) => {
                            if &*val == xsecret {
                              break;
                            }
                            id = id.wrapping_add(1)
                          }
                        }
                      }

                      connected.insert(id, *xsecret, *HEARTBEAT_TIMEOUT).await;

                      if connect_id == id {
                        if let Some(instant) = connecting.expiration(&src_bytes).await {
                          info!("connect cost {:?}", (instant - 3 * *MSL).elapsed());

                          connecting.remove(&src_bytes).await;
                          ipv4_insert(src_bytes)?;
                          unsafe { CONNECTED_TIME = now::sec() };
                        }

                        KAD.write().add((*key).try_into().unwrap(), src);
                        info!("✅ id = {:?}\nxsecret = {:?}", id, xsecret);
                      } else {
                        // TODO 重新连接可以从这一步开始
                        reply!([&cmd_key, &encrypt(xsecret, &id.to_le_bytes())[..]].concat());
                      }
                    } else {
                      reply!([CMD::PONG])
                    }
                  }
                  _ => {
                    error!("CMD::KEY n={}", n)
                  }
                },
                CMD::Q => {
                  if connecting.renew(&src.to_bytes(), *MSL).await {
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
                      let pk = public_key_from_bytes(key);

                      if let Ok(_) = pk.verify_strict(hash, &sign) {
                        KAD.write().add((*key).try_into().unwrap(), src);

                        let xpk: X25519PublicKey = pk.into();
                        let xsecret = x25519_secret.diffie_hellman(&xpk);
                        let xsecret = xsecret.as_bytes();
                        // 设置id
                        // 响应加密后的id
                        let mut hash32 = XxHash32::default();
                        hash32.write(xsecret);
                        println!("xsecret {:?}", xsecret);
                        let mut connect_id = hash32.finish() as u32;

                        loop {
                          if match connected.get(&connect_id).await {
                            None => true,
                            Some(val) => {
                              if &*val == xsecret {
                                true
                              } else {
                                connect_id = connect_id.wrapping_add(1);
                                false
                              }
                            }
                          } {
                            connected
                              .insert(connect_id, *xsecret, *HEARTBEAT_TIMEOUT)
                              .await;
                            let id = encrypt(xsecret, &connect_id.to_le_bytes());
                            reply!([&cmd_key, &id[..]].concat());
                            break;
                          }
                        }
                      }
                    }
                  };
                }
                _ => {
                  info!("{}  > {} : {:?}", src, input[0], &input[1..]);
                  continue;
                }
              }

*/
