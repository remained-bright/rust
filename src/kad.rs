use crate::util::addr_to_bytes::ToBytes;
use crate::util::bytes_to_addr::v4;
use crate::util::same_prefix::same_prefix;
use crate::var::len::PUBLIC_KEY_LENGTH;
use hashbrown::HashMap;
use retainer::Cache;
use smallvec::{smallvec, SmallVec};
use static_init::dynamic;
use std::net::{Ipv4Addr, SocketAddrV4};
const RETURN_SIZE: usize = 128;
const BUCKET_SIZE: usize = RETURN_SIZE * 2;

#[derive(Default)]
pub struct Kad {
  id: [u8; PUBLIC_KEY_LENGTH],
  bucket: SmallVec<[SmallVec<[[u8; 6]; BUCKET_SIZE]>; 256]>,
  exist: HashMap<Ipv4Addr, u8>,
  pub len: usize,
}

// 定期pop最后一个，有响应就会插入，没有响应就会丢弃

#[dynamic]
pub static mut KAD: Kad = Kad::default();

// leading_zeros

impl Kad {
  pub fn add(&mut self, key: [u8; PUBLIC_KEY_LENGTH], ip_port: SocketAddrV4) {
    let ip = ip_port.ip();
    if let None = self.exist.get(ip) {
      let distance = same_prefix(key, self.id) as usize;

      self.exist.insert(ip.clone(), distance as u8);

      if distance == 256 {
        return;
      }

      let mut len = self.bucket.len();
      if len == 0 {
        self.bucket.push(smallvec![ip_port.to_bytes()]);
        self.len = 1;
        return;
      }

      len -= 1;

      if distance > len {
        self.len += 1;
        let bucket = &mut self.bucket[len];
        if bucket.len() < BUCKET_SIZE {
          bucket.insert(0, ip_port.to_bytes());
        } else {
          self.split(ip_port);
        }
      } else {
        let bucket = &mut self.bucket[distance];
        if bucket.len() < BUCKET_SIZE {
          self.len += 1;
          bucket.insert(0, ip_port.to_bytes());
        }
      }
    }
  }

  fn split(&mut self, ip_port: SocketAddrV4) {
    let len = self.bucket.len();
    let mut bucket1 = SmallVec::new();
    let mut bucket2 = smallvec![ip_port.to_bytes()];

    if let Some(bucket) = self.bucket.pop() {
      for i in bucket {
        let i = i.clone();
        if self.exist[v4(i).ip()] as usize == len {
          bucket1.push(i);
        } else {
          bucket2.push(i);
        }
      }
    }
    self.bucket.push(bucket1);
    self.bucket.push(bucket2);
  }

  pub fn neighbor(&self, key: [u8; PUBLIC_KEY_LENGTH]) -> bool {
    false
  }
}
