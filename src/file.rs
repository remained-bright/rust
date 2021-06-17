use anyhow::Result;
use log::info;
use std::{
  fs::OpenOptions,
  io::{Seek, SeekFrom, Write},
};

/*
创建16GB的文件，仅在使用时才增长
https://users.rust-lang.org/t/create-16gb-file-has-it-grow-only-on-use/46998/2
*/

pub fn test() -> Result<()> {
  const SIZE: u64 = 16 * 1024 * 1024;
  let mut open_options = OpenOptions::new();
  open_options.read(true).write(true).create(true);
  let fp = "/Users/z/Downloads/t/estfile";
  info!("file > {}", fp);
  let mut file = open_options.open(fp)?;
  file.seek(SeekFrom::Start(SIZE - 1))?;
  file.write_all(b"\0")?;
  Ok(())
}
