#[derive(Default)]
pub struct Speed {
  pre: u64,
  now: u64,
}

impl Speed {
  fn incr(&mut self, n: u64) {
    self.now.wrapping_add(n);
  }
  fn diff(&mut self) -> u64 {
    let n = if self.now > self.pre {
      self.now - self.pre
    } else {
      self.now + (u64::MAX - self.pre)
    };
    self.pre = self.now;
    n
  }
}
