use crate::util::same_prefix::same_prefix;
use ed25519_dalek_blake3::PublicKey;
use retainer::Cache;
use smallvec::SmallVec;
use std::net::Ipv4Addr;

pub struct Node {
  addr: Ipv4Addr,
  key: PublicKey,
}

struct Kad {
  id: [u8; 32],
  bucket: SmallVec<[SmallVec<[Node; 8]>; 256]>,
  connecting: Cache<[u8; 6], ()>,
}

// leading_zeros

impl Kad {
  pub fn boot() {}
  pub fn add(node: Node) -> bool {
    false
  }
  pub fn neighbor(&self, key: [u8; 32]) -> bool {
    false
  }
}
