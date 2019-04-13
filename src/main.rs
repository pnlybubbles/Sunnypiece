extern crate time;

mod math;
mod film;
mod geometry;
mod ray;
mod sample;
mod camera;
mod object;

use std::path::Path;
use math::*;
use film::{PPM, Image};
use film::Format as _Format;
use ray::Ray;
use geometry::{Sphere, Geometry};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SPP: usize = 1;
type Format = PPM;

fn main() {
  let sphere = Sphere { position: Vector3::zero(), radius: 1.0 };
  let mut film = Image::new(Vector3::zero(), WIDTH, HEIGHT);
  film.each_mut( |v, x, y| {
    let ray = Ray {
      origin: Vector3::new(0.0, 0.0, 5.0),
      direction: Vector3::new(
        x as f32 / WIDTH as f32 - 0.5,
        y as f32 / HEIGHT as f32 - 0.5,
        -1.0,
      ).normalize(),
    };
    match sphere.intersect(&ray) {
      Some(i) => *v = i.normal / 2.0 + Vector3::new(0.5, 0.5, 0.5),
      None => *v = Vector3::zero(),
    }
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
