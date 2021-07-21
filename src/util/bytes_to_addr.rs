use std::convert::TryInto;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

pub fn v4(bytes: [u8; 6]) -> SocketAddrV4 {
  SocketAddrV4::new(
    Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]),
    u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
  )
}
