#![allow(dead_code)]

extern crate image;
extern crate rand;
extern crate rand_core;
extern crate rand_mt;
extern crate rayon;
extern crate time;
extern crate tobj;

mod acceleration;
mod camera;
mod film;
mod geometry;
mod integrator;
mod light_transport;
mod loader;
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
use geometry::UUID;
use geometry::{Sphere, Triangle};
use integrator::Integrator;
use light_transport::Radiance;
use material::Material;
use math::*;
use object::Object;
use rand::SeedableRng;
use std::cell::RefCell;
use std::path::Path;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const SPP: usize = 10;
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
    Vector3::new(0.0, 2.0, 15.0),
    Vector3::new(0.0, 1.69522, 14.0476),
    Vector3::new(0.0, 1.0, 0.0),
  );
  let camera = IdealPinhole::new(36.7774 * PI / 180.0, film.aspect(), camera_matrix);
  // let camera_matrix = Matrix4::look_at(
  //   Vector3::new(278.0, 273.0, -800.0),
  //   Vector3::new(278.0, 273.0, 0.0),
  //   Vector3::new(0.0, 1.0, 0.0),
  // );
  // let camera = IdealPinhole::new(39.3077 * PI / 180.0, film.aspect(), camera_matrix);

  // シーン
  let red_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.75, 0.25, 0.25),
  });
  let blue_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.25, 0.25, 0.75),
  });
  let mat: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.75, 0.75, 0.75),
  });
  let grossy1: Box<dyn Material + Send + Sync> = Box::new(material::GGX {
    reflectance: Vector3::new(1.0, 1.0, 1.0),
    roughness: 0.5,
  });
  let grossy2: Box<dyn Material + Send + Sync> = Box::new(material::GGX {
    reflectance: Vector3::new(1.0, 1.0, 1.0),
    roughness: 0.3,
  });
  let ls = 20_f32;
  let le = 10000.0 / ls.powi(2);
  let light_diffuse: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::new(le, le, le),
    albedo: Vector3::zero(),
  });
  // let light_center = Vector3::new(556.0 / 2.0, 548.8 - 20.0, 559.2 / 2.0);
  // let l0 = light_center + Vector3::new(-ls / 2.0, -0.3, -ls / 2.0);
  // let l1 = l0 + Vector3::new(ls, 0.0, 0.0);
  // let l2 = l0 + Vector3::new(0.0, 0.0, ls);
  // let l3 = l0 + Vector3::new(ls, 0.0, ls);
  // let light1 = Object::new(
  //   Box::new(Triangle::new(l0, l1, l2)),
  //   Matrix4::unit(),
  //   &light_diffuse,
  // );
  // let light2 = Object::new(
  //   Box::new(Triangle::new(l2, l1, l3)),
  //   Matrix4::unit(),
  //   &light_diffuse,
  // );
  let mut uuid = UUID::new();
  let mut objects = Vec::new();
  // let cbox = loader::Obj::new(Path::new("models/simple/cbox.obj"));
  // let luminaire = loader::Obj::new(Path::new("models/simple/cbox_luminaire.obj"));
  // let bunny = loader::Obj::new(Path::new("models/bunny/cbox_bunny.obj"));
  // objects.append(&mut cbox.instances(&mat));
  // objects.append(&mut luminaire.instances(&mat));
  // objects.append(&mut bunny.instances(&grossy1));
  let veach_mis = loader::Obj::new(Path::new("models/veach-mis/veach-mis.obj"));
  objects.append(&mut veach_mis.instances(&mat, &mut uuid));

  // 空間構造
  let structure = acceleration::BVH::new(objects);

  // シードの読み込み
  let args: Vec<String> = std::env::args().collect();

  let seed: u64 = if args.len() >= 2 {
    args[1].parse().unwrap()
  } else {
    rand::random()
  };

  // 積分器
  let mut integrator = integrator::Debug::new(&mut film, SPP, seed);
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
