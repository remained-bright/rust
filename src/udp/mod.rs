mod recv_from;
mod timer;
use crate::udp::recv_from::{recv_from, CONNECTED_TIME};
use crate::udp::timer::timer;
use crate::util::now;
use crate::var::msl::MSL;
use anyhow::Result;
use async_std::net::UdpSocket;
use log::error;
use retainer::Cache;
use static_init::dynamic;
use std::time::Duration;

#[dynamic]
static DURATION: u64 = 3;

#[dynamic]
static EXPIRE: u64 = *DURATION + (*MSL).as_secs() + 1;

pub async fn listen(addr: String) -> Result<()> {
  let connecting = Cache::<[u8; 6], ()>::new();

  let socket = UdpSocket::bind(addr).await?;
  let err = futures::join!(
    timer(&socket, &connecting),
    recv_from(&socket, &connecting),
    connecting.monitor(2, 1, Duration::from_secs(*DURATION), &|kvli| {
      //msl秒内有过成功的连接
      if kvli.len() > 0 && (now::sec() - unsafe { CONNECTED_TIME }) <= *EXPIRE {
        for (k, v) in kvli {
          println!("TODO REMOVE {:?} {:?}", k, v)
        }
      }
    }),
  );
  error!("{:?}", err);
  Ok(())
}
