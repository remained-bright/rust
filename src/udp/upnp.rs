use igd::aio::search_gateway;
use log::info;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddrV4};

pub async fn upnp(name: &str, port: u16) {
  if let Ok(gateway) = search_gateway(Default::default()).await {
    println!("gateway {:?}", gateway.addr);
    if let Ok(stream) = TcpStream::connect(gateway.addr) {
      if let Ok(addr) = stream.local_addr() {
        let ip = addr.ip();
        drop(stream);
        if let IpAddr::V4(ip) = ip {
          info!("ip = {:?}", ip);
          if let Err(err) = gateway
            .add_port(
              igd::PortMappingProtocol::UDP,
              port,
              SocketAddrV4::new(ip, port),
              3600,
              name,
            )
            .await
          {
            info!("upnp port mapping failed : {}", err);
          }
        }
      }
    }
  }
}
