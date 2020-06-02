use super::integrator::Integrator;
use super::util;
use fasthash::murmur3;
use film::Film;
use rand::SeedableRng;
use rayon::prelude::*;
use std::ops::{Add, Div};
use std::sync::Mutex;
use RNG;

pub struct ParRow<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
  pub spp: usize,
  seed: u32,
}

impl<'a, Pixel> ParRow<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>, spp: usize, seed: u32) -> ParRow<'b, Pixel> {
    ParRow {
      film: film,
      spp: spp,
      seed: seed,
    }
  }
}

impl<'a, Pixel> Integrator<Pixel> for ParRow<'a, Pixel> {
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

    let size = self.film.data.len();
    let thread_seed = (0..size)
      .map(|i| unsafe {
        std::mem::transmute::<u32, [u8; 4]>(murmur3::hash32_with_seed(
          std::mem::transmute::<usize, [u8; 8]>(i),
          self.seed,
        ))
      })
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
        RNG.with(|rng| *rng.borrow_mut() = RNG::from_seed(thread_seed[index]));

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
