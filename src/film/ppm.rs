use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use super::image::{Image, Save};
use super::image::Format;

pub struct PPM;

impl<T> Save<T> for PPM
  where T: Copy
{
  type Output = [u8; 3];

  fn save(image: &Image<T>, path: &Path, f: &Fn(T) -> [u8; 3]) -> io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(
      format!("P3\n{} {}\n{}\n", image.width, image.height, 255)
        .as_bytes(),
    )?;
    for y in 0..image.height {
      for x in 0..image.width {
        let c = f(image.get(x, y));
        file.write_all(
          format!("{} {} {}\n", c[0], c[1], c[2]).as_bytes(),
        )?;
      }
    }
    Ok(())
  }
}

impl Format for PPM {
  fn ext() -> &'static str {
    "ppm"
  }
}
