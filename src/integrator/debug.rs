use super::integrator::Integrator;
use super::util::ProgressIndicator;
use film::Film;
use rand::{Rng, SeedableRng};
use std::ops::{Add, Div};
use RNG;

pub struct Debug<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
  pub spp: usize,
  seed: u64,
}

impl<'a, Pixel> Debug<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>, spp: usize, seed: u64) -> Debug<'b, Pixel> {
    Debug {
      film: film,
      spp: spp,
      seed: seed,
    }
  }
}

impl<'a, Pixel> Integrator<Pixel> for Debug<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: Clone + Send + Sync + Add<Pixel, Output = Pixel> + Div<f32, Output = Pixel>,
    F: Fn(f32, f32) -> Pixel,
  {
    let chunk_size = self.film.width;
    let spp = self.spp;
    let uv = self.film.uv();
    let total = self.film.height * self.film.width;
    let mut progress = ProgressIndicator::new(total);

    println!("Using seed: {}", self.seed);

    let mut local_rng = RNG::seed_from_u64(self.seed);
    let thread_seed = (0..total)
      .map(|_| local_rng.gen::<u64>())
      .collect::<Vec<_>>();

    self
      .film
      .data
      .iter_mut()
      .enumerate()
      .for_each(|(index, pixel)| {
        progress.next();

        if index % chunk_size == 0 {
          RNG.with(|rng| *rng.borrow_mut() = RNG::seed_from_u64(thread_seed[index / chunk_size]));
        }

        *pixel = (0..spp).fold(pixel.clone(), |sum, _| {
          let (u, v) = uv(index);
          sum + f(u, v)
        }) / spp as f32
      });

    progress.end();
  }
}
