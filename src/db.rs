use crate::args::DIR;
use crate::ed25519::seed_new;
use crate::util::now;
use anyhow::Result;
pub use persy::{ByteVec, Config, Persy, ValueMode};
use static_init::dynamic;
use std::path::{Path, PathBuf};

#[dynamic]
pub static DB_FILE: PathBuf = Path::new(&*DIR).join("rmw.persy");

#[allow(non_upper_case_globals)]
pub mod db {
  pub const config: &str = "config";
  pub const ipv4_time: &str = "ipv4Time";
  pub const time_ipv4: &str = "timeIpv4";
  pub const id_public: &str = "idPublic";
  pub const public_id: &str = "publicId";
}

#[dynamic]
pub static TX: Persy = {
  Persy::open_or_create_with(&*DB_FILE, Config::new(), |p| {
    let mut tx = p.begin()?;
    tx.create_index::<ByteVec, ByteVec>(db::config, ValueMode::Replace)?;
    tx.create_index::<[u8; 6], u64>(db::ipv4_time, ValueMode::Replace)?;
    tx.create_index::<u64, [u8; 6]>(db::time_ipv4, ValueMode::Cluster)?;
    //tx.create_segment(db::ipv4)?;
    tx.commit()?;
    Ok(())
  })
  .unwrap()
};

pub fn seed() -> Result<[u8; 32]> {
  let key: ByteVec = "seed".into();
  Ok(match TX.one::<ByteVec, ByteVec>(db::config, &key)? {
    Some(s) => (*s).try_into().unwrap(),
    None => {
      let s = seed_new();
      TX.put(db::config, key, ByteVec::new(s.to_vec()))?;
      s
    }
  })
}

pub fn ipv4_insert(addr: [u8; 6]) -> Result<bool> {
  let mut now = now::sec();
  let mut tx = TX.begin()?;

  if let Some(v) = tx.one::<_, u64>(db::ipv4_time, &addr)? {
    if v <= now {
      return Ok(false);
    }
    now = u64::min(v >> 1, now);
    tx.remove(db::time_ipv4, v, Some(addr))?;
  }

  tx.put(db::ipv4_time, addr, now)?;
  tx.put(db::time_ipv4, now, addr)?;

  tx.commit()?;
  Ok(true)
}

const MAX_TIME: u64 = (!0) >> 1;

pub fn ipv4_offline(addr: [u8; 6]) -> Result<()> {
  let mut tx = TX.begin()?;
  let mut time: u64 = MAX_TIME;

  for t in tx.get::<_, u64>(db::ipv4_time, &addr)? {
    time = t;
    tx.remove(db::time_ipv4, time, Some(addr))?;
  }
  if time >= MAX_TIME {
    tx.remove::<_, u64>(db::ipv4_time, addr, None)?;
  } else {
    time = time << 1;
    tx.put(db::time_ipv4, time, addr)?;
    tx.put(db::ipv4_time, addr, time)?;
  }

  tx.commit()?;
  Ok(())
}
