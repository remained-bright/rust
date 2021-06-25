use ed25519_dalek_blake3::{PublicKey, SecretKey, PUBLIC_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use std::convert::Into;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

trait FnMove {
  fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnMove for F {
  fn call(self: Box<Self>) {
    (*self)()
  }
}

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

  let mut stop = false;

  for id in 0..4 {
    let t_seed_s = mpsc::Sender::clone(&seed_s);
    let t_count_s = mpsc::Sender::clone(&count_s);
    let closure: Box<dyn FnMove + Send> = unsafe {
      std::mem::transmute(Box::new(|| {
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
          if n % 100 == 0 {
            if stop {
              return;
            }
            if n % 10000 == 0 {
              println!("{} > {}", id, n / 10000);
            }
            t_count_s.send(true).unwrap();
          }

          let bytes = public.as_bytes();
          if bytes[PUBLIC_KEY_LENGTH - 1] == 0 && bytes[PUBLIC_KEY_LENGTH - 2] == 0 {
            println!("seed {:?}\npublic {:?}", s, public.as_bytes());
            t_seed_s.send(seed.arr).unwrap();
            stop = true;
            return;
          }
        }
      }) as Box<dyn FnMove>)
    };
    thread::spawn(move || closure.call());
  }

  let mut count = 0;
  for _ in count_r {
    count += 1;
    if count % 100 == 0 {
      println!("count = {}", count / 100);
    }
  }

  let seed = seed_r.recv().unwrap();

  println!("seed = {:?}", seed);
  println!("cost time {}", now.elapsed().as_secs());
}
