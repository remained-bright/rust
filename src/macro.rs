#[macro_export]
macro_rules! Await {
  ($s:expr) => {
    match $s.await {
      Err(err) => error!("{:?}", err),
      _ => {}
    }
  };
}
