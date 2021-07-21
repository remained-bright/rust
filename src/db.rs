use crate::args::DIR;
use persy::{Config, Persy, ValueMode};
use static_init::dynamic;
use std::path::{Path, PathBuf};

#[dynamic]
static DB_FILE: PathBuf = Path::new(&*DIR).join("rmw.persy");

pub mod str {
  pub const index: &str = "index";
}

#[dynamic]
static DB: Persy = {
  Persy::open_or_create_with(&*DB_FILE, Config::new(), |p| {
    let mut tx = p.begin()?;
    tx.create_index::<u64, u64>(str::index, ValueMode::Replace)?;
    tx.create_segment("seg")?;
    tx.prepare()?.commit()?;
    Ok(())
  })
  .unwrap()
};
