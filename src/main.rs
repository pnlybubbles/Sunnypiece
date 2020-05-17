#![allow(dead_code)]

extern crate rayon;
extern crate time;

mod acceleration;
mod camera;
mod film;
mod geometry;
mod integrator;
mod light_transport;
mod material;
mod math;
mod object;
mod ray;
mod sample;
mod sampler;
mod util;

use camera::{Camera, IdealPinhole};
use film::Format;
use film::{Film, Save, PPM};
use geometry::Sphere;
use integrator::Integrator;
use light_transport::Radiance;
use math::*;
use object::Object;
use std::path::Path;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SPP: usize = 1;
type Image = PPM;

fn main() {
  // フィルム
  let mut film = Film::new(Vector3::zero(), WIDTH, HEIGHT);

  // カメラ
  let camera = IdealPinhole::new(PI / 2.0, film.aspect(), Matrix4::unit());

  // シーン
  let light_material = material::Lambertian {
    emittance: Vector3::new(1.0, 1.0, 1.0),
    albedo: Vector3::zero(),
  };
  let white_material = material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(1.0, 1.0, 1.0),
  };
  let light = Object::new(
    Box::new(Sphere {
      position: Vector3::new(0.0, 0.0, -5.0),
      radius: 1.0,
    }),
    Matrix4::unit(),
    Box::new(light_material),
  );
  let sphere = Object::new(
    Box::new(Sphere {
      position: Vector3::new(0.0, -1001.0, -5.0),
      radius: 1000.0,
    }),
    Matrix4::unit(),
    Box::new(white_material),
  );
  let objects = vec![light, sphere];

  // 空間構造
  let structure = acceleration::Linear::new(objects);

  {
    // 積分器
    let mut integrator = integrator::ParPixel::new(&mut film, 10000);
    // 光輸送
    let light_transporter = light_transport::Naive {
      structure: structure,
    };

    integrator.each(|u, v| {
      let ray = camera.sample(u, v);
      light_transporter.radiance(&ray.value)
    });
  }

  // 保存
  let file_path = &format!(
    "images/image_{}_{}.{}",
    time::now().strftime("%Y%m%d%H%M%S").unwrap(),
    SPP,
    Image::ext(),
  );
  Image::save(&film, Path::new(&file_path), |v| {
    let correct = v.map(|v| v.min(1.0).max(0.0) * 255.0);
    [correct.x as u8, correct.y as u8, correct.z as u8]
  })
  .unwrap();
}
