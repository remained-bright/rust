use crate::args::DIR;
use crate::kad::BUCKET_SIZE;
use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use static_init::dynamic;
use std::include_bytes;
use std::path::{Path, PathBuf};

#[dynamic]
pub static DB_FILE: PathBuf = Path::new(&*DIR).join("rmw.db");

#[dynamic]
pub static POOL: Pool<SqliteConnectionManager> =
  Pool::new(SqliteConnectionManager::file(&*DB_FILE)).unwrap();

pub fn init() -> Result<()> {
  let c = POOL.get()?;
  if 0
    == c.query_row(
      "SELECT count(1) FROM sqlite_master WHERE type='table'",
      [],
      |row| row.get::<_, usize>(0),
    )?
  {
    c.execute_batch(&unsafe { String::from_utf8_unchecked(include_bytes!("db.sql").to_vec()) })?;
  }
  Ok(())
}

pub fn site_ipv4(site_id: u64) -> Result<Vec<[u8; 6]>> {
  let vec = Vec::new();
  let c = POOL.get()?;
  let mut q = c.prepare(
    format!(
      "select ip,port from site_ipv4 where site_id=? order by rank limit {}",
      BUCKET_SIZE
    )
    .as_str(),
  )?;
  let mut iter = q.query(params![site_id])?;
  while let Some(row) = iter.next()? {
    println!("{} {}", row.get::<_, u32>(0)?, row.get::<_, u16>(1)?);
  }

  Ok(vec)
}

/*
use crate::args::DIR;
use crate::util::now;
pub use persy::{ByteVec, Config, Persy, ValueMode};
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
*/
