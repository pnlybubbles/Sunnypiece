#![allow(dead_code)]

extern crate image;
extern crate rand;
extern crate rand_core;
extern crate rand_mt;
extern crate rayon;
extern crate scarlet;
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
use film::{tonemap, Film, Format, Save, Validate, PNG};
use geometry::UUID;
use integrator::Integrator;
use light_transport::Radiance;
use material::Material;
use math::*;
use rand::SeedableRng;
use std::cell::RefCell;
use std::path::Path;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
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
  let mat: Box<dyn Material + Send + Sync> = Box::new(material::Lambertian {
    emittance: Vector3::zero(),
    albedo: Vector3::new(0.75, 0.75, 0.75),
  });
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
  // let tonemap = tonemap::Debug {
  //   colormap: scarlet::colormap::ListedColorMap::viridis(),
  //   clamp: (0.0, 1.0),
  // };
  let tonemap = tonemap::Gamma::default();
  Image::save(&film, Path::new(&file_path), tonemap)
}
