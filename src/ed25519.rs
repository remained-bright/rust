use ed25519_dalek_blake3::{PublicKey, SecretKey};
use rand_core::{OsRng, RngCore};
use std::mem::MaybeUninit;
use std::time::Instant;

pub fn rand<const N: usize>() -> [u8; N] {
  let mut arr: [u8; N] = unsafe { MaybeUninit::uninit().assume_init() };
  (OsRng {}).fill_bytes(&mut arr[..]);
  arr
}

pub fn seed() {
  let now = Instant::now();
  let mut n = 0;

  let mut secret;
  let mut public: PublicKey;

  loop {
    let seed = rand::<32>();

    secret = SecretKey::from_bytes(&seed).unwrap();
    public = (&secret).into();

    //let (_, body, _) = unsafe { public_bytes.align_to::<u32>() };
    //println!("encode bytes: {}", body.len());

    n += 1;
    if n % 50000 == 0 {
      println!("{}", n / 50000);
    }

    let bytes = public.as_bytes();
    if bytes[0] == 0 && bytes[1] == 0 {
      println!("seed {:?}\npublic {:?}", seed, public.as_bytes());
      break;
    }
  }

  println!("n={} cost time {}", n, now.elapsed().as_secs());
}
