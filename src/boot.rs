//use crate::db::DB;
use crate::ed25519::seed;
use crate::{grpc, udp, ws};
use log::{error, info};
use rand::{thread_rng, Rng};
use std::net::UdpSocket;
use var_byte_str::VarByteString;

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
  use var_byte_str::VarByteString;
  let original = "Some really long text and may contains some different language like \"คำภาษาไทยที่ใช้พื้นที่เยอะกว่าเนื้อความภาษาอังกฤษเสียอีก\".";
  let encoded = VarByteString::from(original);
  println!("The text is {}", encoded);
  println!("UTF-8 took {} bytes", original.as_bytes().len());
  println!("Internal structure is {:?}", encoded);

  seed();
  //init_sqlite().unwrap();
  //info!("> {:?}", std::env::current_exe().unwrap().parent().unwrap());
  let err = futures::join!(
    listen!(udp, {
      let ip = "0.0.0.0";
      let mut rng = thread_rng();

      let port = {
        let mut p: u16 = rng.gen_range(1025..10000);
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
