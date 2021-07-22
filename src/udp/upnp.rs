use async_std::task::sleep;
use igd::aio::search_gateway;
use log::info;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddrV4};
use std::time::Duration;

pub async fn upnp(name: &str, port: u16, duration: u32) -> bool {
  if let Ok(gateway) = search_gateway(Default::default()).await {
    if let Ok(stream) = TcpStream::connect(gateway.addr) {
      if let Ok(addr) = stream.local_addr() {
        let ip = addr.ip();
        drop(stream);
        if let IpAddr::V4(ip) = ip {
          if let Err(err) = gateway
            .add_port(
              igd::PortMappingProtocol::UDP,
              port,
              SocketAddrV4::new(ip, port),
              duration,
              name,
            )
            .await
          {
            info!("upnp failed : {}", err);
          } else {
            return true;
          }
        }
      }
    }
  }
  false
}

const SLEEP_SECONDS: u32 = 60;

pub async fn upnp_daemon(name: &str, port: u16) {
  let seconds = Duration::from_secs(SLEEP_SECONDS.into());
  loop {
    upnp(name, port, SLEEP_SECONDS + 60).await;
    sleep(seconds).await;
  }
}
