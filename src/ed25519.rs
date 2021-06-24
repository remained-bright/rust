use ed25519_dalek_blake3::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;
use static_init::dynamic;
use std::convert::TryInto;
use std::time::{Duration, Instant};

#[dynamic]
static PREFIX: [u8; 8] = "rmw.link".as_bytes().try_into().unwrap();

pub fn seed() {
  let mut rng = OsRng {};
  let prefix = *PREFIX;
  let len = prefix.len();

  let now = Instant::now();

  let mut secret_key;
  let mut public_key: PublicKey;

  loop {
    secret_key = SecretKey::generate(&mut rng);
    public_key = (&secret_key).into();
    if public_key.as_bytes()[..len] == prefix {
      break;
    }
  }

  println!("cost time {}", now.elapsed().as_secs());

  println!("public {:?}", public_key.as_bytes()[..len] == prefix);
}
