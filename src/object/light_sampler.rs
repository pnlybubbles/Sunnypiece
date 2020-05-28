use geometry::Geometry;
use math::*;
use object::Object;
use rand::Rng;
use sample::{pdf, Sample};
use RNG;

pub struct LightSampler<'a> {
  light: Vec<&'a Object<'a>>,
  light_area: f32,
}

impl<'a> LightSampler<'a> {
  pub fn new(objects: &'a Vec<Object>) -> Self {
    // 光源だけ取り出す
    let light = objects
      .iter()
      .filter(|v| v.material.emittance().sqr_norm() > 0.0)
      .collect::<Vec<_>>();
    // 光源の表面積を計算
    let light_area = light.iter().map(|v| v.geometry.area()).sum();
    LightSampler {
      light: light,
      light_area: light_area,
    }
  }

  pub fn sample(&self) -> Sample<Vector3, pdf::Area> {
    // 面積のみを考慮した光源の重点的サンプリング
    // NOTE: 位置ベクトルがサンプリングされる
    RNG.with(|rng| {
      let r = rng.borrow_mut().gen::<f32>();
      let roulette = self.light_area * r;
      let mut area = 0.0;
      for obj in &self.light {
        area += obj.geometry.area();
        if roulette <= area {
          let sample = obj.geometry.sample();
          return Sample {
            value: sample.value,
            pdf: sample.pdf * (obj.geometry.area() / self.light_area),
          };
        }
      }
      unreachable!();
    })
  }

  pub fn pdf(&self, geometry: &Box<dyn Geometry + Send + Sync>) -> pdf::Area {
    geometry.pdf() * (geometry.area() / self.light_area)
  }
}
