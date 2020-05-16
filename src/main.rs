extern crate time;

mod acceleration;
mod camera;
mod film;
mod geometry;
mod integrator;
mod math;
mod object;
mod ray;
mod sample;
mod util;

use camera::{Camera, IdealPinhole};
use film::Format;
use film::{Film, Save, PPM};
use geometry::{Geometry, Sphere};
use integrator::{DebugIntegrator, Integrator};
use math::*;
use object::{Object, Scene};
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

  let scene = Scene {
    camera: camera,
    structure: structure,
  };

  {
    // 積分器
    let mut integrator = DebugIntegrator::new(&mut film);

    integrator.each(|apply, u, v| {
      // 光輸送
      debug_assert!(u.less_than_unit(), "0 <= u < 1.0");
      debug_assert!(v.less_than_unit(), "0 <= v < 1.0");
      let ray = scene.camera.sample(u, v);
      debug_assert!(
        ray.value.direction.is_normalized(),
        "ray direction should be normalized."
      );
      let color = match scene.structure.intersect(&ray.value) {
        Some(i) => i.normal.to_color(),
        None => Vector3::zero(),
      };
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
