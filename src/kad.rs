use ed25519_dalek_blake3::PublicKey;
use smallvec::SmallVec;
use std::net::Ipv4Addr;

pub struct Node {
  addr: Ipv4Addr,
  key: PublicKey,
}

struct Kad {
  bucket: Vec<SmallVec<[Node; 16]>>,
}

impl Kad {
  pub fn add(node: Node) -> bool {
    false
  }
  pub fn neighbor(node: Node) -> bool {
    false
  }
}
