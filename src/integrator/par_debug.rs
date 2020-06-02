use super::integrator::Integrator;
use super::util;
use film::Film;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::ops::{Add, Div};
use std::sync::Mutex;
use RNG;

pub struct ParDebug<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
  pub spp: usize,
  seed: u64,
}

impl<'a, Pixel> ParDebug<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>, spp: usize, seed: u64) -> ParDebug<'b, Pixel> {
    ParDebug {
      film: film,
      spp: spp,
      seed: seed,
    }
  }
}

impl<'a, Pixel> Integrator<Pixel> for ParDebug<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: Clone + Send + Sync + Add<Pixel, Output = Pixel> + Div<f32, Output = Pixel>,
    F: Send + Sync + Fn(f32, f32) -> Pixel,
  {
    let chunk_size = self.film.width;
    let spp = self.spp;
    let uv = self.film.uv();
    let total = self.film.height * self.film.width / chunk_size;
    let progress = Mutex::new(0);

    println!("Using seed: {}", self.seed);

    let mut local_rng = RNG::seed_from_u64(self.seed);
    let thread_seed = (0..total)
      .map(|_| local_rng.gen::<u64>())
      .collect::<Vec<_>>();

    self
      .film
      .data
      .par_chunks_mut(chunk_size)
      .enumerate()
      .for_each(|(index, slice)| {
        {
          let mut p = progress.lock().unwrap();
          *p += 1;
          util::progress_indicator(*p, total);
        }

        // initialize thread local rng
        RNG.with(|rng| *rng.borrow_mut() = RNG::seed_from_u64(thread_seed[index]));

        // heavy task
        slice.iter_mut().enumerate().for_each(|(i, pixel)| {
          *pixel = (0..spp).fold(pixel.clone(), |sum, _| {
            let (u, v) = uv(index * chunk_size + i);
            sum + f(u, v)
          }) / spp as f32
        })
      });
    println!("");
  }
}
