extern crate time;

mod math;
mod film;

use std::path::Path;
use math::*;
use film::{PPM, Image};
use film::Format as _Format;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SPP: usize = 1;
type Format = PPM;

fn main() {
  let mut film = Image::new(Vector3::zero(), WIDTH, HEIGHT);
  film.each_mut( |v, x, y| {
    *v = Vector3::new(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32, 1.0);
  });
  let file_path = &format!(
    "images/image_{}_{}.{}",
    time::now().strftime("%Y%m%d%H%M%S").unwrap(),
    SPP,
    Format::ext(),
  );
  film.save::<Format>(Path::new(&file_path), &|v| {
    let correct = v.map( &|v| v.min(1.0).max(0.0) * 255.0 );
    [correct.x as u8, correct.y as u8, correct.z as u8]
  }).unwrap();
}
