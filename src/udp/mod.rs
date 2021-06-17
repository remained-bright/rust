pub mod addr_to_bytes;
mod recv_from;
mod timer;
use crate::udp::recv_from::recv_from;
use crate::udp::timer::timer;
use anyhow::Result;
use async_std::net::UdpSocket;
use log::error;
use retainer::Cache;
use std::time::Duration;

pub async fn listen(addr: String) -> Result<()> {
  let connecting = Cache::<[u8; 6], ()>::new();

  let socket = UdpSocket::bind(addr).await?;
  let err = futures::join!(
    timer(&socket, &connecting),
    recv_from(&socket, &connecting),
    connecting.monitor(2, 0.5, Duration::from_secs(3)),
  );
  error!("{:?}", err);
  Ok(())
}
