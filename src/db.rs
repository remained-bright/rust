use crate::args::DIR;
use crate::util::now;
use anyhow::Result;
use persy::{Config, Persy, ValueMode};
use static_init::dynamic;
use std::path::{Path, PathBuf};

#[dynamic]
static DB_FILE: PathBuf = Path::new(&*DIR).join("rmw.persy");

#[allow(non_upper_case_globals)]
pub mod str {
  pub const ipv4_time: &str = "ipv4Time";
  pub const time_ipv4: &str = "timeIpv4";
}

#[dynamic]
static TX: Persy = {
  Persy::open_or_create_with(&*DB_FILE, Config::new(), |p| {
    let mut tx = p.begin()?;
    tx.create_index::<[u8; 6], u64>(str::ipv4_time, ValueMode::Replace)?;
    tx.create_index::<u64, [u8; 6]>(str::time_ipv4, ValueMode::Cluster)?;
    //tx.create_segment(str::ipv4)?;
    tx.commit()?;
    Ok(())
  })
  .unwrap()
};

pub fn ipv4_insert(addr: [u8; 6]) -> Result<()> {
  let now = now::sec();
  let mut tx = TX.begin()?;

  if let Some(_) = tx.get::<[u8; 6], u64>(str::ipv4_time, &addr)?.next() {
    println!("{:?} exists", addr);
    return Ok(());
  }

  tx.put(str::ipv4_time, addr, now)?;
  tx.put(str::time_ipv4, now, addr)?;

  tx.commit()?;
  Ok(())
}
