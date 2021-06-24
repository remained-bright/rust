use ed25519_dalek_blake3::{PublicKey, SecretKey};
use rand_core::{OsRng, RngCore};
use std::mem::MaybeUninit;
use std::time::Instant;

const PREFIX: [u8; 2] = [0, 0];

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
  let len = PREFIX.len();

  let now = Instant::now();
  let mut n = 0;

  for seed in ArrIncr::new() {
    let secret = SecretKey::from_bytes(&seed).unwrap();
    let public: PublicKey = (&secret).into();
    n += 1;
    if n % 10000 == 0 {
      println!("n = {} seed = {:?}", n, seed);
    }
    if public.as_bytes()[..len] == PREFIX {
      println!("seed {:?}\npublic {:?}", seed, public.as_bytes());
      break;
    }
  }

  println!("cost time {}", now.elapsed().as_secs());
}
