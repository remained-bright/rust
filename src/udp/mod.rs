mod recv_from;
mod timer;
use crate::udp::recv_from::{recv_from, CONNECTED_TIME};
use crate::udp::timer::timer;
use crate::util::now;
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
    connecting.monitor(2, 1, Duration::from_secs(3), &|kvli| {
      if kvli.len() > 0 && now::sec() - unsafe { CONNECTED_TIME } <= 6 {
        //6秒内有过成功的连接
        for (k, v) in kvli {
          println!("{:?} {:?}", k, v)
        }
      }
    }),
  );
  error!("{:?}", err);
  Ok(())
}
