use super::integrator::Integrator;
use super::util::ProgressIndicator;
use film::Film;
use rayon::prelude::*;
use std::ops::{Add, Div};
use std::sync::Mutex;

pub struct ParPixel<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
  pub spp: usize,
}

impl<'a, Pixel> ParPixel<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>, spp: usize) -> ParPixel<'b, Pixel> {
    ParPixel {
      film: film,
      spp: spp,
    }
  }
}

impl<'a, Pixel> Integrator<Pixel> for ParPixel<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: Clone + Send + Sync + Add<Pixel, Output = Pixel> + Div<f32, Output = Pixel>,
    F: Send + Sync + Fn(f32, f32) -> Pixel,
  {
    let spp = self.spp;
    let uv = self.film.uv();
    let total = self.film.height * self.film.width;
    let progress = Mutex::new(ProgressIndicator::new(total));

    self
      .film
      .data
      .par_iter_mut()
      .enumerate()
      .for_each(|(index, pixel)| {
        {
          progress.lock().unwrap().next();
        }
        // heavy task
        *pixel = (0..spp).fold(pixel.clone(), |sum, _| {
          let (u, v) = uv(index);
          sum + f(u, v)
        }) / spp as f32
      });

    progress.lock().unwrap().end();
  }
}
