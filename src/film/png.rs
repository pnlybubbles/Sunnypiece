use super::film::Format;
use super::film::{Film, Save};
use std::fs::File;
use std::path::Path;

pub struct PNG;

impl<T> Save<T> for PNG
where
  T: Copy,
{
  type Output = [u8; 3];

  fn save(film: &Film<T>, path: &Path, f: impl Fn(&T) -> Self::Output) {
    let mut buf = image::ImageBuffer::new(film.width as u32, film.height as u32);
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
      let output_pixel = film.get(x as usize, y as usize);
      *pixel = image::Rgb(f(output_pixel));
    }
    let ref mut file = File::create(path).expect("ERROR! could not open file.");
    image::DynamicImage::ImageRgb8(buf)
      .write_to(file, image::ImageFormat::Png)
      .expect("ERROR! failed to write image.")
  }
}

impl Format for PNG {
  fn ext() -> &'static str {
    "png"
  }
}
