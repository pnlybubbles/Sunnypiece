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
use geometry::{Geometry, Sphere};
use integrator::{DebugIntegrator, Integrator};
use light_transport::Radiance;
use math::*;
use object::Object;
use std::path::Path;
use util::*;

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
  let sphere = Object::new(
    Box::new(Sphere {
      position: Vector3::new(0.0, 0.0, -5.0),
      radius: 1.0,
    }),
    Matrix4::unit(),
  );
  let objects = vec![sphere];

  // 空間構造
  let structure = acceleration::Linear::new(objects);

  {
    // 積分器
    let mut integrator = DebugIntegrator::new(&mut film);
    // 光輸送
    let light_transporter = light_transport::Normal {
      structure: structure,
    };

    integrator.each(|apply, u, v| {
      let ray = camera.sample(u, v);
      let color = light_transporter.radiance(&ray.value);
      apply(color)
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
