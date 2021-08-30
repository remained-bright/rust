//use crate::db::{db, TX};
use crate::kad::KAD;
use crate::udp::state::SPEED;
use crate::util::addr_to_bytes::ToBytes;
use crate::util::bytes_to_addr;
use crate::var::cmd::CMD;
use crate::var::duration::MSL;
use async_std::net::UdpSocket;
use async_std::task::sleep;
use log::{error, info};
use retainer::Cache;
use std::time::Duration;
use std::{net::SocketAddrV4, str::FromStr};

fn error_tip(ip: &str) {
  error!("config error : can't parse {:?} ip", ip);
}

pub async fn boot(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) {
  macro_rules! send {
    ($ip:expr) => {
      info!("ping {:?}", $ip);
      match socket.send_to(&[CMD::PING], $ip).await {
        Err(err) => info!("ipv4 ping error {}", err),
        Ok(_) => {
          connecting.insert($ip.to_bytes(), (), *MSL).await;
        }
      };
    };
  }
  /*
    for (n, (_, li)) in TX
      .range::<u64, [u8; 6], _>(db::time_ipv4, ..)
      .unwrap()
      .enumerate()
    {
      for ipv4 in li {
        let ipv4 = bytes_to_addr::v4(ipv4);
        send!(ipv4);
      }
      if n == 128 {
        return;
      }
    }
    for ip in (config_get!(boot_ipv4, {
      "47.104.79.244:32342 54.177.127.37:8616".to_string()
    }))
    .split(' ')
    {
      match SocketAddrV4::from_str(ip) {
        Ok(v4) => {
          if let Some(_) = TX.one::<_, u64>(db::ipv4_time, &v4.to_bytes()).unwrap() {
            continue;
          }
          send!(v4);
        }
        _ => error_tip(ip),
      }
    }
  */
}

pub async fn timer(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) {
  // 可能网络故障导致连接失败，所以每10秒尝试一次重新连接
  boot(socket, connecting).await;
  let duration = Duration::from_secs(3);
  loop {
    unsafe { SPEED.diff() }
    sleep(duration).await;
    println!("kad len {}", KAD.read().len);
    println!("speed {}", unsafe { SPEED.speed });
  }
  /*
  let mut interval = stream::interval(Duration::from_secs(10));
  while let Some(_) = interval.next().await {
    boot(socket, connecting).await
  }
  */
}
