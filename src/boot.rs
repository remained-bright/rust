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
  let original = "前天下午，一边在家欣赏梅艳芳的《女人花》，一边观看大华、歌尔、蓝标等长牛股的跳水表演。想起曾经风华绝代的梅姑年方40就香消玉殒，看着数年前一骑绝尘的大华和歌尔如今股价节节败退，不由心生感慨，于是写下这样一段文字：\"由来只有新人笑，有谁听到旧人哭\"，一句简单地歌词，很贴切地描绘出近期的市场特征。成长股犹如女明星，大部分吃的都是青春饭，如果既年轻又漂亮，再配上些才艺，就很容易受追捧。一旦年暮色衰，哪怕你再有实力，也很难有巨大的市场号召力。投资成长股，跟演艺公司培养女星似有异曲同工之妙";
  let encoded = VarByteString::from(original);
  println!("The text is {}", encoded);
  println!("UTF-8 took {} bytes", original.as_bytes().len());
  println!("UTF-8 took {} bytes", original);
  println!(
    "Internal structure is {} {}",
    encoded.buffer_len(),
    encoded.sign_len()
  );

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
