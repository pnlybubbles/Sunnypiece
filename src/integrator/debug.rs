use super::integrator::Integrator;
use film::Image;

pub struct DebugIntegrator<'a, Pixel> {
  pub film: &'a mut Image<Pixel>,
}

impl<'a, Pixel> DebugIntegrator<'a, Pixel> {
  fn new<'b>(film: &'b mut Image<Pixel>) -> DebugIntegrator<'b, Pixel> {
    DebugIntegrator { film: &mut film }
  }
}

impl<'a, Pixel> Integrator<Pixel> for DebugIntegrator<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    F: Fn(&mut FnMut(Pixel), f32, f32),
  {
    let film = &mut self.film;
    for y in 0..film.height {
      for x in 0..film.width {
        let u = x as f32 / film.width as f32;
        let v = y as f32 / film.height as f32;
        f(&mut |value| film.data[y][x] = value, u, v)
      }
    }
  }
}
