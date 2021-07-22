#![feature(async_closure)]
#![feature(ptr_internals)]
#![feature(array_zip)]
#![feature(let_chains)]

#[macro_use]
mod r#macro;

#[macro_use]
mod config;

mod args;
mod db;
mod ed25519;
mod file;
mod grpc;
mod kad;
mod udp;
mod util;
mod var;
mod ws;

//#[allow(non_upper_case_globals)]
//mod kv;

mod boot;
mod init;

#[async_std::main]
async fn main() {
  init::init();
  boot::boot().await;
}
