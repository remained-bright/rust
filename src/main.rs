#[macro_use]
mod r#macro;

#[macro_use]
mod config;

mod args;
mod file;
mod grpc;
mod udp;
mod var;
mod ws;

//#[allow(non_upper_case_globals)]
//mod kv;

mod boot;
mod init;

use log::info;

#[async_std::main]
async fn main() {
  init::init();
  boot::boot().await;
}
