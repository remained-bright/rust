use rand::{thread_rng, Rng};
use std::net::{Ipv4Addr, UdpSocket};

pub fn port_available(port: u16) -> bool {
  match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, port)) {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn find_port() -> u16 {
  let mut rng = thread_rng();
  let p: u16 = rng.gen_range(9001..30000);
  (p..65535)
    .find(|port| port_available(*port))
    .expect("no udp port available")
}
