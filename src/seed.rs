use crate::args::DIR;
use crate::ed25519::seed_new;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

pub fn seed() -> [u8; 32] {
  let p = Path::new(&*DIR).join("seed");
  if p.exists() {
    let mut data = Vec::new();

    File::open(&p).unwrap().read_to_end(&mut data).unwrap();
    if let Ok(r) = data.try_into() {
      return r;
    }
  }
  let seed = seed_new();
  BufWriter::new(File::create(&p).unwrap())
    .write_all(&seed)
    .unwrap();
  seed
}
