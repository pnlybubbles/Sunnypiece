use super::film::Format;
use super::film::{Film, Save};
use super::tonemap::Tonemap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct PPM;

impl<T> Save<T> for PPM
where
  T: Copy,
{
  type Output = [u8; 3];

  fn save<M>(film: &Film<T>, path: &Path, tonemap: M)
  where
    M: Tonemap<Input = T, Output = Self::Output>,
  {
    let f = tonemap.mapper(film);
    let mut file = File::create(path).expect("ERROR! could not open file.");
    file
      .write_all(format!("P3\n{} {}\n{}\n", film.width, film.height, 255).as_bytes())
      .expect("ERROR! failed to write image.");
    for y in 0..film.height {
      for x in 0..film.width {
        let c = f(film.get(x, y));
        file
          .write_all(format!("{} {} {}\n", c[0], c[1], c[2]).as_bytes())
          .expect("ERROR! failed to write image.");
      }
    }
  }
}

impl Format for PPM {
  fn ext() -> &'static str {
    "ppm"
  }
}
