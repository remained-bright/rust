use clap::clap_app;
use static_init::dynamic;

//use std::convert::TryInto;
//use std::net::SocketAddr;
//use wickdb::{ReadOptions, WriteOptions, DB};

#[dynamic]
pub static ARGS: clap::ArgMatches<'static> = clap_app!(
  app =>
    (version: "0.0.1")
    (@arg dir: -d --dir +takes_value "dir")
)
.get_matches();

#[dynamic]
pub static DIR: String = match ARGS.value_of("dir") {
  Some(d) => d.to_string(),
  _ => {
    // env::current_dir().unwrap().display().to_string()
    let mut p = dirs::home_dir().unwrap();
    p.push(".rmw");
    std::fs::create_dir_all(&p).unwrap();
    p.display().to_string()
  }
};
