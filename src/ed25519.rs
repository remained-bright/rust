use ed25519_dalek_blake3::{PublicKey, SecretKey, PUBLIC_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use std::mem::MaybeUninit;
use std::time::Instant;

struct ArrIncr<const N: usize> {
  pos: usize,
  pub arr: [u8; N],
}

impl<const N: usize> ArrIncr<N> {
  pub fn new() -> Self {
    let mut arr: [u8; N] = unsafe { MaybeUninit::uninit().assume_init() };
    (OsRng {}).fill_bytes(&mut arr[..]);
    ArrIncr { pos: 0, arr: arr }
  }
}

impl<const N: usize> Iterator for ArrIncr<N> {
  type Item = [u8; N];
  fn next(&mut self) -> Option<[u8; N]> {
    let pos = self.pos;
    self.arr[pos] = u8::wrapping_add(self.arr[pos], 1);
    self.pos = (pos + 1) % N;
    Some(self.arr)
  }
}

pub fn seed() {
  let now = Instant::now();
  let mut n = 0;

  for seed in ArrIncr::<32>::new() {
    let secret = SecretKey::from_bytes(&seed).unwrap();
    let public: PublicKey = (&secret).into();
    let bytes = public.as_bytes();

    n += 1;
    if n % 10000 == 0 {
      println!("{} : public {:?}", n, bytes);
    }

    if bytes[0] == bytes[PUBLIC_KEY_LENGTH - 2] && bytes[1] == bytes[PUBLIC_KEY_LENGTH - 1] {
      println!("seed {:?}\npublic {:?}", seed, public.as_bytes());
      break;
    }
  }

  println!("n={} cost time {}", n, now.elapsed().as_secs());
}
