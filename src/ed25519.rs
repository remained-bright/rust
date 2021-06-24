use ed25519_dalek_blake3::{Keypair, PublicKey, SecretKey};
use rand_core::{OsRng, RngCore};
use static_init::dynamic;
use std::convert::TryInto;
use std::time::{Duration, Instant};

#[dynamic]
static PREFIX: [u8; 2] = "rm".as_bytes().try_into().unwrap();

struct ArrIncr {
  pos: usize,
  begin: u8,
  now: [u8; 32],
}

impl ArrIncr {
  /*
  pub fn new() -> Self {
    let mut arr = [0u8; 32];
    (OsRng {}).fill_bytes(&mut arr[..]);

    //  ArrIncr {}
  }
  */
}

pub fn arr_incr(seed: &mut [u8; 32]) {
  let mut i = seed.len();

  while i != 0 {
    i -= 1;
    let n = seed[i];
    if n == 255 {
      seed[i] = 0;
    } else {
      seed[i] = n + 1;
      return;
    }
  }
}

pub fn seed() {
  let mut rng = OsRng {};
  let prefix = *PREFIX;
  let len = prefix.len();

  let now = Instant::now();

  let secret_key = SecretKey::generate(&mut rng);
  let seed = secret_key.as_bytes();
  let mut public_key: PublicKey;
  let mut n = 0;
  println!("seed {:?}", seed);
  /*
  loop {
    let secret = SecretKey::from_bytes(seed).unwrap();
    public_key = (&secret).into();
    n += 1;
    break;
    if n % 10000 == 0 {
      println!("n = {}", n);
    }
    if public_key.as_bytes()[..len] == prefix {
      break;
    }
  }

  println!("cost time {}", now.elapsed().as_secs());

  println!("public {:?}", public_key.as_bytes()[..len] == prefix);
  */
}
