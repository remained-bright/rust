mod recv_from;
mod timer;
mod upnp;

use crate::db::ipv4_offline;
use crate::udp::recv_from::{recv_from, CONNECTED_TIME};
use crate::udp::timer::timer;
use crate::util::now;
use crate::var::duration::{HEARTBEAT, MSL};
use anyhow::Result;
use async_std::net::UdpSocket;
use log::error;
use retainer::Cache;
use static_init::dynamic;
use std::time::Duration;

#[dynamic]
static EXPIRE: u64 = (*MSL).as_secs() + 1;

pub async fn listen(addr: String) -> Result<()> {
  let connected = Cache::<u32, [u8; 32]>::new();
  let connecting = Cache::<[u8; 6], ()>::new();
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
    timer(&socket, &connecting),
    recv_from(&socket, &connecting, &connected),
    connected.monitor(2, 1, *HEARTBEAT, &|_| {}),
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
  );
  error!("{:?}", err);
  Ok(())
}
