use async_std::task::sleep;
use igd::aio::search_gateway;
use log::info;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::time::Duration;

pub async fn upnp(name: &str, port: u16, duration: u32) -> Option<Ipv4Addr> {
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
            return Some(ip);
          }
        }
      }
    }
  }
  None
}

const SLEEP_SECONDS: u32 = 60;

pub async fn upnp_daemon(name: &str, port: u16) {
  let mut local_ip = Ipv4Addr::UNSPECIFIED;

  let seconds = Duration::from_secs(SLEEP_SECONDS.into());

  loop {
    if let Some(ip) = upnp(name, port, SLEEP_SECONDS + 60).await {
      if ip != local_ip {
        local_ip = ip;
        info!("upnp success ( {}:{} )", local_ip, port);
      }
    };
    sleep(seconds).await;
  }
}
