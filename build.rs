//use std::env;
use std::error::Error;
//use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
  /*
  tonic_build::configure()
    .type_attribute("routeguide.Point", "#[derive(Hash)]")
    .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
    .unwrap();
  */

  tonic_build::configure()
    .build_client(false)
    .compile(&["proto/rmw.proto"], &["proto"])?;
  Ok(())
}
