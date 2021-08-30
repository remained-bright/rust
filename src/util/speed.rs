#[derive(Default)]
pub struct Speed {
  pub pre: usize,
  pub n: usize,
  pub speed: usize,
}

impl Speed {
  pub fn incr(&mut self, n: usize) {
    self.n = self.n.wrapping_add(n);
  }
  pub fn diff(&mut self) {
    self.speed = if self.n < self.pre {
      self.n + (usize::MAX - self.pre)
    } else {
      self.n - self.pre
    };
    self.pre = self.n;
  }
}
