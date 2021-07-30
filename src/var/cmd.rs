#[non_exhaustive]
pub struct CMD;

impl CMD {
  pub const PING: u8 = 1;
  pub const PONG: u8 = 2;
  pub const KEY: u8 = 3;
  pub const Q: u8 = 4;
  pub const A: u8 = 5;
  pub const ID: u8 = 6;
}
