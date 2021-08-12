use ed25519_dalek_blake3::{PublicKey, SecretKey, PUBLIC_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use std::convert::Into;
use std::convert::TryInto;
use std::io::prelude::*;
use std::mem::MaybeUninit;
use std::sync::mpsc;
use std::thread;
//use std::time::Instant;

struct Seed {
  pub arr: [u8; 32],
  rng: OsRng,
}

impl Seed {
  pub fn new() -> Self {
    Self {
      arr: unsafe { MaybeUninit::uninit().assume_init() },
      rng: OsRng {},
    }
  }
  pub fn next(&mut self) -> &[u8] {
    self.rng.fill_bytes(&mut self.arr);
    &self.arr
  }
}

pub fn _seed(c: mpsc::Sender<Option<[u8; 32]>>) {
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
      if n % 500 == 0 {
        match c.send(None) {
          Err(_) => {
            return;
          }
          _ => {}
        }
      }

      let bytes = public.as_bytes();
      if bytes[PUBLIC_KEY_LENGTH - 1] == 0 && bytes[PUBLIC_KEY_LENGTH - 2] == 0 {
        c.send(Some(seed.arr)).unwrap_or(());
        return;
      }
    }
  });
}

pub fn seed_new() -> [u8; 32] {
  //let now = Instant::now();

  println!("首次运行，生成秘钥中，请稍等 ···");

  let (seed_s, seed_r) = mpsc::channel();

  let thread_num = num_cpus::get() - 1;

  for _ in 1..thread_num {
    _seed(mpsc::Sender::clone(&seed_s))
  }
  _seed(seed_s);

  let mut count = 0;
  loop {
    match seed_r.recv().unwrap() {
      None => {
        count += 1;
        if count % thread_num == 0 {
          print!(".");
          std::io::stdout().flush().unwrap();
        }
      }
      Some(seed) => {
        print!("✅\n");
        return seed;
      }
    }
  }
}
