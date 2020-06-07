#![allow(dead_code)]

extern crate image;
extern crate rand;
extern crate rand_core;
extern crate rand_mt;
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
use film::{Film, Save, Validate, PNG};
use geometry::{Sphere, Triangle};
use integrator::Integrator;
use light_transport::Radiance;
use material::Material;
use math::*;
use object::Object;
use rand::SeedableRng;
use std::cell::RefCell;
use std::path::Path;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SPP: usize = 100;
type Image = PNG;
type RNG = rand::rngs::StdRng;

thread_local! {
  pub static RNG: RefCell<RNG> = RefCell::new(SeedableRng::from_entropy());
}

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
  let grossy1: Box<dyn Material + Send + Sync> = Box::new(material::GGX {
    reflectance: Vector3::new(1.0, 1.0, 1.0),
    roughness: 0.8,
    ior: 100000.0,
  });
  let grossy2: Box<dyn Material + Send + Sync> = Box::new(material::GGX {
    reflectance: Vector3::new(1.0, 1.0, 1.0),
    roughness: 0.3,
    ior: 100000.0,
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
    Box::new(Sphere::new(Vector3::new(-4.0, -room_size + 3.0, 0.0), 3.0)),
    Matrix4::unit(),
    &grossy1,
  );
  let sphere2 = Object::new(
    Box::new(Sphere::new(Vector3::new(4.0, -room_size + 3.0, 0.0), 3.0)),
    Matrix4::unit(),
    &grossy2,
  );
  let ls = 3_f32;
  let le = 675.0 / ls.powi(2);
  let light_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::new(le, le, le),
    albedo: Vector3::zero(),
  });
  let l0 = Vector3::new(-ls / 2.0, room_size - 0.5, -ls / 2.0);
  let l1 = l0 + Vector3::new(ls, 0.0, 0.0);
  let l2 = l0 + Vector3::new(0.0, 0.0, ls);
  let l3 = l0 + Vector3::new(ls, 0.0, ls);
  let light1 = Object::new(
    Box::new(Triangle::new(l0, l1, l2)),
    Matrix4::unit(),
    &light_diffuse,
  );
  let light2 = Object::new(
    Box::new(Triangle::new(l2, l1, l3)),
    Matrix4::unit(),
    &light_diffuse,
  );
  let objects = vec![
    sphere1, sphere2, top, bottom, left, right, back, light1, light2,
  ];
  // let objects = vec![back];

  // 空間構造
  let structure = acceleration::Linear::new(objects);

  // シードの読み込み
  let args: Vec<String> = std::env::args().collect();

  let seed: u64 = if args.len() >= 2 {
    args[1].parse().unwrap()
  } else {
    rand::random()
  };

  // 積分器
  let mut integrator = integrator::ParPixel::new(&mut film, SPP);
  // 光輸送
  let light_transporter = light_transport::ExplicitLight::new(&structure);

  integrator.each(|u, v| {
    let ray = camera.sample(u, v);
    light_transporter.radiance(ray.value)
  });

  // NAN, INFINITY チェック
  film.validate();

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
}
