use ed25519_dalek_blake3::{PublicKey, SecretKey, PUBLIC_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use std::convert::Into;
use std::io::prelude::*;
use std::mem::MaybeUninit;
use std::ptr::Unique;
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

pub fn _seed(c: mpsc::Sender<Option<[u8; 32]>>, stop: Unique<bool>) {
  let stop = unsafe { &mut *stop.as_ptr() };
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
      if n % 10 == 0 {
        if *stop {
          return;
        }
        if n % 500 == 0 {
          c.send(None).unwrap();
        }
      }

      let bytes = public.as_bytes();
      if bytes[PUBLIC_KEY_LENGTH - 1] == 0 && bytes[PUBLIC_KEY_LENGTH - 2] == 0 {
        *stop = true;
        c.send(Some(seed.arr)).unwrap();
        return;
      }
    }
  });
}

pub fn seed() {
  let now = Instant::now();

  println!("首次运行，生成秘钥中，请稍等 ···");

  let (seed_s, seed_r) = mpsc::channel();

  // Rust：线程间共享数据 https://zhuanlan.zhihu.com/p/37760452
  let stop = Unique::from(Box::leak(Box::new(false)));

  let thread_num = num_cpus::get() - 1;

  for _ in 1..thread_num {
    _seed(mpsc::Sender::clone(&seed_s), stop)
  }
  _seed(seed_s, stop);

  let mut count = 0;
  for i in seed_r {
    match i {
      None => {
        count += 1;
        if count % thread_num == 0 {
          print!(".");
          std::io::stdout().flush().unwrap();
        }
      }
      Some(seed) => {
        println!("");

        unsafe {
          Box::from_raw(stop.as_ptr());
        }

        println!("seed = {:?}", seed);
        println!("cost time {}", now.elapsed().as_secs());
        return;
      }
    }
  }
}
