use super::integrator::Integrator;
use film::Film;

pub struct DebugIntegrator<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
}

impl<'a, Pixel> DebugIntegrator<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>) -> DebugIntegrator<'b, Pixel> {
    DebugIntegrator { film: film }
  }
}

impl<'a, Pixel> Integrator<Pixel> for DebugIntegrator<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    F: Fn(&mut dyn FnMut(Pixel), f32, f32),
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
