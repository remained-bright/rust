use ed25519_dalek_blake3::{PublicKey, SecretKey, PUBLIC_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use std::convert::Into;
use std::mem::MaybeUninit;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

struct Seed {
  pub arr: [u8; 32],
}

impl Seed {
  pub fn new() -> Self {
    Self {
      arr: unsafe { MaybeUninit::uninit().assume_init() },
    }
  }
  pub fn next(&mut self) -> &[u8] {
    (OsRng {}).fill_bytes(&mut self.arr[..]);
    &self.arr
  }
}

pub fn seed() {
  let now = Instant::now();

  println!("首次运行，生成秘钥中，请稍等 ···");

  let (seed_s, seed_r) = mpsc::channel();
  let (count_s, count_r) = mpsc::channel();

  //  let tx1 = mpsc::Sender::clone(&tx);

  thread::spawn(move || {
    let mut seed = Seed::new();
    let mut secret;
    let mut n = 0;
    let mut public: PublicKey;

    loop {
      let s = seed.next();
      secret = SecretKey::from_bytes(s).unwrap();
      public = (&secret).into();

      //let (_, body, _) = unsafe { public_bytes.align_to::<u32>() };
      //println!("encode bytes: {}", body.len());

      n += 1;
      if n % 1000 == 0 {
        count_s.send(1).unwrap();
      }

      let bytes = public.as_bytes();
      if bytes[PUBLIC_KEY_LENGTH - 1] == 0 && bytes[PUBLIC_KEY_LENGTH - 2] == 0 {
        println!("seed {:?}\npublic {:?}", s, public.as_bytes());
        seed_s.send(seed.arr).unwrap();
        return;
      }
    }
  });

  let mut count = 0;
  for _ in count_r {
    count += 1;
    if count % 10 == 0 {
      println!("count = {}", count / 10);
    }
  }

  let seed = seed_r.recv().unwrap();

  println!("seed = {:?}", seed);
  println!("cost time {}", now.elapsed().as_secs());
}
