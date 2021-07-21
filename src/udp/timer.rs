use crate::db::{str, TX};
use crate::util::addr_to_bytes::ToBytes;
use crate::util::bytes_to_addr;
use crate::var::cmd::CMD;
use crate::var::msl::MSL;
use async_std::net::UdpSocket;
use log::info;
use retainer::Cache;
use std::net::SocketAddrV4;

pub async fn boot(socket: &UdpSocket, connecting: &Cache<[u8; 6], ()>) {
  for (_, li) in TX.range::<u64, [u8; 6], _>(str::time_ipv4, ..).unwrap() {
    for ipv4 in li {
      println!("ipv4 {:?}", bytes_to_addr::v4(ipv4));
    }
  }

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
  // 可能网络故障导致连接失败，所以每10秒尝试一次重新连接
  boot(socket, connecting).await;
  /*
  let mut interval = stream::interval(Duration::from_secs(10));
  while let Some(_) = interval.next().await {
    boot(socket, connecting).await
  }
  */
}
