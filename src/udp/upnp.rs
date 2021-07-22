use anyhow::Result;
use igd::aio::search_gateway;
use igd::PortMappingProtocol;
use std::env;
use std::net::SocketAddrV4;

pub async fn upnp(addr: SocketAddrV4) -> Result<()> {
  let gateway = match search_gateway(Default::default()).await {
    Ok(g) => g,
    Err(err) => {
      println!("Faild to find IGD: {}", err);
      return Ok(());
    }
  };

  Ok(())
}
