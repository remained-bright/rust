use crate::util::same_prefix::same_prefix;
use hashbrown::HashSet;
use retainer::Cache;
use smallvec::SmallVec;
use std::net::{Ipv4Addr, SocketAddrV4};

struct Kad {
  id: [u8; 32],
  bucket: SmallVec<[SmallVec<[[u8; 32]; 1024]>; 256]>,
  exist: HashSet<Ipv4Addr>,
  connecting: Cache<[u8; 6], ()>,
}

// leading_zeros

impl Kad {
  pub fn boot() {}
  pub fn add(&mut self, key: [u8; 32], ip_port: SocketAddrV4) {
    let ip = ip_port.ip();
    if let Some(_) = self.exist.get(ip) {
      self.exist.insert(ip.clone());
      let n = same_prefix(key, self.id);
      if (n as usize) > self.bucket.len() {}
    }
  }
  pub fn neighbor(&self, key: [u8; 32]) -> bool {
    false
  }
}
