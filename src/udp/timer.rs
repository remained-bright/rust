use crate::udp::addr_to_bytes::ToBytes;
use crate::var::cmd::CMD;
use crate::var::msl::MSL;
use async_std::net::UdpSocket;
use async_std::prelude::*;
use async_std::stream;
use log::{error, info};
use retainer::Cache;
use std::net::SocketAddrV4;
use std::time::Duration;

pub async fn boot(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) {
  for ip in (config_get!(boot_ipv4, {
    "47.104.79.244:32342 54.177.127.37:29040".to_string()
  }))
  .split(' ')
  {
    if ip.chars().count() >= 8 {
      //connecting.insert(ip, );
      let ip: SocketAddrV4 = ip.parse().unwrap();
      match socket.send_to(&[CMD::PING], ip).await {
        Err(err) => info!("{}", err),
        Ok(_) => {
          connecting.insert(ip.to_bytes(), (), *MSL).await;
        }
      };
    }
  }
}

pub async fn timer(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) {
  boot(socket, connecting).await;
  let mut interval = stream::interval(Duration::from_secs(10));
  while let Some(_) = interval.next().await {
    boot(socket, connecting).await
  }
}
