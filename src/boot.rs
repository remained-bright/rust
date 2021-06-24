//use crate::db::DB;
use crate::ed25519::seed;
use crate::{grpc, udp, ws};
use log::{error, info};
use rand::{thread_rng, Rng};
use std::net::UdpSocket;

macro_rules! listen {
  ($func:ident, $default: block) => {{
    let addr = config_get!($func, $default);
    info!("{}://{}", stringify!($func), addr);
    $func::listen(addr)
  }};
}

/*
pub fn init_sqlite() -> Result<()> {
  info!("sqlite version {}", oneString!("select sqlite_version()"));
  Ok(())
}
*/

pub async fn boot() {
  seed();
  //init_sqlite().unwrap();
  //info!("> {:?}", std::env::current_exe().unwrap().parent().unwrap());
  let err = futures::join!(
    listen!(udp, {
      let ip = "0.0.0.0";
      let mut rng = thread_rng();

      let port = {
        let mut p: u16 = rng.gen_range(3000..9000);
        loop {
          if let Ok(_) = UdpSocket::bind(format!("{}:{}", ip, p)) {
            break p;
          } else {
            p += 1;
          }
        }
      };

      format!("{}:{}", ip, port)
    }),
    listen!(grpc, { "0.0.0.0:2080".to_string() }),
    listen!(ws, { "0.0.0.0:2081".to_string() }),
  );
  error!("{:?}", err);
}
