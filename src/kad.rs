use crate::util::addr_to_bytes::ToBytes;
use crate::util::same_prefix::same_prefix;
use hashbrown::HashMap;
use retainer::Cache;
use smallvec::SmallVec;
use std::net::{Ipv4Addr, SocketAddrV4};

struct Kad {
  id: [u8; 32],
  bucket: SmallVec<[SmallVec<[[u8; 6]; 512]>; 256]>,
  exist: HashMap<Ipv4Addr, u8>,
  connecting: Cache<[u8; 6], ()>,
}

// leading_zeros

impl Kad {
  pub fn boot() {}
  pub fn add(&mut self, key: [u8; 32], ip_port: SocketAddrV4) {
    let ip = ip_port.ip();
    if let Some(_) = self.exist.get(ip) {
      let distance = same_prefix(key, self.id) as usize;

      self.exist.insert(ip.clone(), distance as u8);

      if distance >= 256 {
        return;
      }

      let mut len = self.bucket.len();
      if len == 0 {
        let mut bucket = SmallVec::new();
        bucket.push(ip_port.to_bytes());
        self.bucket.push(bucket);
        return;
      }

      len -= 1;

      if distance > len {
        let bucket = &mut self.bucket[distance];
        let bucket_len = bucket.len();
        if bucket_len >= 32 {
          //split
        } else {
          bucket.insert(0, ip_port.to_bytes());
        }
      } else {
        let bucket = &mut self.bucket[distance];
        let bucket_len = bucket.len();
        if bucket_len >= 32 {
          // test
        } else {
          bucket.insert(0, ip_port.to_bytes());
        }
      }
    }
  }
  pub fn neighbor(&self, key: [u8; 32]) -> bool {
    false
  }
}
