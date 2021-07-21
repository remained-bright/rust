use static_init::dynamic;
use std::time::Duration;

#[dynamic]
// MSL (Maximum Segment Lifetime) : 报文段最大生存时间，这里用作连接超时时间
pub static MSL: Duration = {
  let mut msl = config_get!(msl, { 6.to_string() }).parse().unwrap();
  if msl < 1 {
    msl = 1
  }
  Duration::from_secs(msl)
};
