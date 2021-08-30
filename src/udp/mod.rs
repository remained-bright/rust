mod recv_from;
mod state;
mod timer;
mod upnp;

use crate::udp::recv_from::{recv_from, CONNECTED_TIME};
use crate::udp::timer::timer;
use crate::util::now;
use crate::var::duration::{HEARTBEAT_TIMEOUT, MSL};
use anyhow::Result;
use async_std::net::UdpSocket;
use log::error;
use log::info;
use retainer::Cache;
use static_init::dynamic;
use std::time::Duration;

#[dynamic]
static EXPIRE: u64 = (*MSL).as_secs() + 1;

pub async fn listen(addr: String) -> Result<()> {
  let socket = UdpSocket::bind(addr).await?;

  println!("{:?}", socket.local_addr().unwrap());

  let err = futures::join!(
    (async || {
      if let Ok(true) = config_get!(upnp, { true.to_string() }).parse() {
        if let Ok(addr) = socket.local_addr() {
          upnp::upnp_daemon("rmw", addr.port()).await
        }
      }
    })(),
    timer(&socket),
    recv_from(&socket),
    /*
    connected.monitor(2, 1, *HEARTBEAT_TIMEOUT, &|kvli| {
      if kvli.len() > 0 {
        for (k, v) in kvli {
          info!("{} {:?} HEARTBEAT EXPIRE", k, v)
        }
      }
    }),
    connecting.monitor(2, 1, *MSL / 3 + Duration::from_secs(1), &|kvli| {
      //msl秒内有过成功的连接
      if kvli.len() > 0 && (now::sec() - unsafe { CONNECTED_TIME }) <= *EXPIRE {
        for (k, _) in kvli {
          ipv4_offline(*k)
            .map_err(|err| error!("ipv4_offline {}", err))
            .unwrap_or(());
        }
      }
    }),
    */
  );
  error!("{:?}", err);
  Ok(())
}
