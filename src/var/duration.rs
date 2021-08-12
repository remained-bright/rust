use static_init::dynamic;
use std::time::Duration;

#[dynamic]
// MSL (Maximum Segment Lifetime) : 报文段最大生存时间，这里用作连接超时时间
pub static MSL: Duration = {
  let mut msl = config_get!(msl, { 3.to_string() }).parse().unwrap();
  if msl < 1 {
    msl = 1
  }
  Duration::from_secs(msl)
};

#[dynamic]
// UDP 心跳超时
pub static HEARTBEAT: u64 = {
  let mut heartbeat = config_get!(heartbeat, { 19.to_string() }).parse().unwrap();
  if heartbeat < 1 {
    heartbeat = 1
  }
  heartbeat
};

#[dynamic]
// UDP 心跳超时
pub static HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(3 + *HEARTBEAT);
