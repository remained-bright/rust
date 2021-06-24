//use crate::db::DB;
use crate::ed25519::seed;
use crate::{grpc, udp, ws};
use log::{error, info};
use rand::Rng;

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
      format!("0.0.0.0:{}", rand::thread_rng().gen_range(10000..30000))
    }),
    listen!(ws, { "0.0.0.0:9980".to_string() }),
    listen!(grpc, { "0.0.0.0:9981".to_string() }),
  );
  error!("{:?}", err);
}
