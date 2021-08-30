//use crate::db::DB;
use crate::db;
use crate::util::find_port::find_port;
use crate::{grpc, udp, ws};
use log::{error, info};

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
  //init_sqlite().unwrap();
  //info!("> {:?}", std::env::current_exe().unwrap().parent().unwrap());
  // db::init().await;
  db::init().unwrap();
  let err = futures::join!(
    listen!(udp, { format!("0.0.0.0:{}", find_port()) }),
    listen!(grpc, { "0.0.0.0:2080".to_string() }),
    listen!(ws, { "0.0.0.0:2081".to_string() }),
  );
  error!("{:?}", err);
}
