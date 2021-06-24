use ed25519_dalek_blake3::{PublicKey, SecretKey};
use rand_core::{OsRng, RngCore};
use static_init::dynamic;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::time::Instant;

#[dynamic]
static PREFIX: [u8; 2] = "rm".as_bytes().try_into().unwrap();

struct ArrIncr {
  pos: usize,
  pub arr: [u8; 32],
}

impl ArrIncr {
  pub fn new() -> Self {
    let mut arr: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    (OsRng {}).fill_bytes(&mut arr[..]);
    ArrIncr { pos: 0, arr: arr }
  }
}

impl Iterator for ArrIncr {
  type Item = [u8; 32];
  fn next(&mut self) -> Option<[u8; 32]> {
    let pos = self.pos;
    self.arr[pos] = u8::wrapping_add(self.arr[pos], 1);
    self.pos = (pos + 1) % self.arr.len();
    Some(self.arr)
  }
}

pub fn seed() {
  let prefix = *PREFIX;
  let len = prefix.len();

  let now = Instant::now();

  let mut public_key: PublicKey;
  let mut n = 0;
  let mut secret;

  for seed in ArrIncr::new() {
    secret = SecretKey::from_bytes(&seed).unwrap();
    public_key = (&secret).into();
    n += 1;
    if n % 10000 == 0 {
      println!("n = {} seed = {:?}", n, seed);
    }
    if public_key.as_bytes()[..len] == prefix {
      println!("public {:?}", public_key.as_bytes()[..len] == prefix);
      break;
    }
  }

  println!("cost time {}", now.elapsed().as_secs());
}
