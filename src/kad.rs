use std::net::Ipv4Addr;

struct Kad {
  bucket: Vec<Node>,
}

pub struct Node {
  addr: Ipv4Addr,
  key: [u8; 32],
}

impl Kad {
  pub fn add(node: Node) -> bool {
    false
  }
}
