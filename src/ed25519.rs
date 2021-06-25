use ed25519_dalek_blake3::{PublicKey, SecretKey, PUBLIC_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use std::mem::MaybeUninit;
use std::time::Instant;

struct Seed<const N: usize> {
  arr: [u8; N],
}

impl<const N: usize> Seed<N> {
  pub fn new() -> Self {
    Self {
      arr: unsafe { MaybeUninit::uninit().assume_init() },
    }
  }
  pub fn next(&mut self) -> [u8; N] {
    (OsRng {}).fill_bytes(&mut self.arr[..]);
    self.arr
  }
}

pub fn seed() {
  let now = Instant::now();
  let mut n = 0;

  let mut secret;
  let mut public: PublicKey;

  println!("首次运行，生成秘钥中，请稍等 ···");

  let mut seed = Seed::<32>::new();

  loop {
    let s = seed.next();
    secret = SecretKey::from_bytes(&s).unwrap();
    public = (&secret).into();

    //let (_, body, _) = unsafe { public_bytes.align_to::<u32>() };
    //println!("encode bytes: {}", body.len());

    n += 1;
    if n % 50000 == 0 {
      println!("{}", n / 50000);
    }

    let bytes = public.as_bytes();
    if bytes[PUBLIC_KEY_LENGTH - 1] == 0 && bytes[PUBLIC_KEY_LENGTH - 2] == 0 {
      println!("seed {:?}\npublic {:?}", s, public.as_bytes());
      break;
    }
  }

  println!("n={} cost time {}", n, now.elapsed().as_secs());
}
