use crate::args::DIR;
use sanakirja::*;
use static_init::dynamic;
use std::fs::create_dir_all;
use std::path::Path;

/*
use sled::{Db, Tree};

#[dynamic]
pub static KV: Db = sled::Config::new()
  .path(Path::new(&*DIR).join("kv"))
  //.use_compression(true)
  .cache_capacity(256 * 1024 * 1024) // 256 MB
  .open()
  .unwrap();

#[dynamic]
pub static id: Tree = KV.open_tree("id").unwrap();

#[dynamic]
pub static ipv4Time: Tree = KV.open_tree("ipv4Time").unwrap();

*/
