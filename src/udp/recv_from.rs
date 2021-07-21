use crate::db::ipv4_insert;
use crate::udp::addr_to_bytes::ToBytes;
use crate::util::now;
use crate::var::cmd::CMD;
use crate::var::msl::MSL;
use anyhow::Result;
use async_std::net::UdpSocket;
use bytes::BytesMut;
use log::{error, info};
use retainer::Cache;
use static_init::dynamic;
use std::net::SocketAddr::V4;

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
            if n == 0 {
              let key = src.to_bytes();
              match connecting.expiration(&key).await {
                Some(instant) => {
                  info!(
                    "ip pong {} connecting elapsed {:?}",
                    src,
                    (instant - *MSL).elapsed()
                  );
                  connecting.remove(&key).await;
                  ipv4_insert(key)?;

                  unsafe { CONNECTED_TIME = now::sec() };
                }
                None => {}
              }
            } else {
              let data = &input[1..];
              match input[0] {
                CMD::PING => reply!([]),
                _ => {
                  println!("{}  > {}", src, input[0]);
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
