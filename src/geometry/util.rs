pub struct UUID {
  i: usize,
}

impl UUID {
  pub fn new() -> Self {
    UUID { i: 0 }
  }

  pub fn gen(&mut self) -> usize {
    self.i += 1;
    self.i
  }
}
