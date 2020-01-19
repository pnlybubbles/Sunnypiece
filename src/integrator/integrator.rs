pub trait Integrator<Pixel> {
  fn each<F>(&mut self, f: F)
  where
    F: Fn(&mut FnMut(Pixel), f32, f32);
}
