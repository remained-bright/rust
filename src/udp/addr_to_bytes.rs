use std::net::SocketAddrV4;
//use bytes::{BufMut, Bytes, BytesMut};

pub trait ToBytes<const N: usize> {
  fn to_bytes(&self) -> [u8; N];
}

impl ToBytes<6> for SocketAddrV4 {
  fn to_bytes(&self) -> [u8; 6] {
    let o = self.ip().octets();
    let p = self.port().to_le_bytes();
    [o[0], o[1], o[2], o[3], p[0], p[1]]
  }
}
/*
impl ToBytes<10> for SocketAddrV6 {
  fn to_bytes(&self) -> [u8; 10] {
    let o = self.ip().octets();
    let p = self.port().to_le_bytes();
    [o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7], p[0], p[1]]
  }
}
*/
