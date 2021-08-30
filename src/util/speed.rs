#[derive(Default)]
pub struct Speed {
  pub pre: usize,
  pub now: usize,
}

impl Speed {
  pub fn incr(&mut self, n: usize) {
    self.now = self.now.wrapping_add(n);
  }
  pub fn diff(&mut self) -> usize {
    let n = if self.now > self.pre {
      self.now - self.pre
    } else {
      self.now + (usize::MAX - self.pre)
    };
    self.pre = self.now;
    n
  }
}
