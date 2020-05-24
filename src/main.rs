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
use material::Material;
use math::*;
use object::Object;
use std::path::Path;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SPP: usize = 100;
type Image = PPM;

fn main() {
  // フィルム
  let mut film = Film::new(Vector3::zero(), WIDTH, HEIGHT);

  // カメラ
  let camera_matrix = Matrix4::look_at(
    Vector3::new(0.0, 0.0, 35.0),
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
  );
  let camera = IdealPinhole::new(39.6 * PI / 180.0, film.aspect(), camera_matrix);

  // シーン
  let light_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::new(5.0, 5.0, 5.0),
    albedo: Vector3::zero(),
  });
  let red_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.75, 0.25, 0.25),
  });
  let blue_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.25, 0.25, 0.75),
  });
  let white_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.75, 0.75, 0.75),
  });
  let room_size = 10.0;
  let left = Object::new(
    Box::new(Sphere::new(Vector3::new(-1e4 - room_size, 0.0, 0.0), 1e4)),
    Matrix4::unit(),
    &red_diffuse,
  );
  let right = Object::new(
    Box::new(Sphere::new(Vector3::new(1e4 + room_size, 0.0, 0.0), 1e4)),
    Matrix4::unit(),
    &blue_diffuse,
  );
  let back = Object::new(
    Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1e4 - room_size), 1e4)),
    Matrix4::unit(),
    &white_diffuse,
  );
  let bottom = Object::new(
    Box::new(Sphere::new(Vector3::new(0.0, -1e4 - room_size, 0.0), 1e4)),
    Matrix4::unit(),
    &white_diffuse,
  );
  let top = Object::new(
    Box::new(Sphere::new(Vector3::new(0.0, 1e4 + room_size, 0.0), 1e4)),
    Matrix4::unit(),
    &white_diffuse,
  );
  let sphere1 = Object::new(
    Box::new(Sphere::new(Vector3::new(0.0, -room_size + 3.0, 0.0), 3.0)),
    Matrix4::unit(),
    &white_diffuse,
  );
  // let light = Object::new(
  //   Box::new(Sphere::new(
  //     Vector3::new(0.0, room_size + 200.0 - 0.1, 0.0),
  //     200.0,
  //   )),
  //   Matrix4::unit(),
  //   &light_diffuse,
  // );
  let light = Object::new(
    Box::new(Sphere::new(Vector3::new(0.0, room_size - 2.5, 0.0), 2.0)),
    Matrix4::unit(),
    &light_diffuse,
  );
  let objects = vec![sphere1, top, bottom, left, right, back, light];

  // 空間構造
  let structure = acceleration::Linear::new(objects);

  // 積分器
  let mut integrator = integrator::ParPixel::new(&mut film, SPP);
  // 光輸送
  let light_transporter = light_transport::Naive::new(&structure);

  integrator.each(|u, v| {
    let ray = camera.sample(u, v);
    light_transporter.radiance(ray.value)
  });

  // 保存
  let file_path = &format!(
    "images/image_{}_{}.{}",
    time::now().strftime("%Y%m%d%H%M%S").unwrap(),
    SPP,
    Image::ext(),
  );
  Image::save(&film, Path::new(&file_path), |v| {
    // ガンマ補正
    let gamma = 2.2;
    let correct = v.map(|v| v.min(1.0).max(0.0).powf(1.0 / gamma) * 255.0);
    [correct.x as u8, correct.y as u8, correct.z as u8]
  })
  .unwrap();
}
