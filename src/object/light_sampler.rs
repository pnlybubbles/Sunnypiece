use geometry::Geometry;
use math::*;
use object::Object;
use rand::Rng;
use sample::{pdf, Sample};
use RNG;

pub struct LightSampler<'a> {
  light: Vec<&'a Object<'a>>,
  pdf: Vec<f32>,
}

impl<'a> LightSampler<'a> {
  pub fn new(objects: &'a Vec<Object>) -> Self {
    // 光源だけ取り出す
    let light = objects
      .iter()
      .filter(|v| v.material.emittance().sqr_norm() > 0.0)
      .collect::<Vec<_>>();
    let intensity = light
      .iter()
      .map(|v| v.geometry.area() * v.material.emittance().max())
      .collect::<Vec<_>>();
    let normalize_factor: f32 = intensity.iter().sum();
    let pdf = intensity
      .iter()
      .map(|v| v / normalize_factor)
      .collect::<Vec<_>>();
    LightSampler { light, pdf }
  }

  /**
   * 光源の重点的サンプリング
   *
   * NOTE: 位置ベクトルがサンプリングされる
   */
  pub fn sample(&self) -> Option<Sample<Vector3, pdf::Area>> {
    RNG.with(|rng| {
      let roulette = rng.borrow_mut().gen::<f32>();
      let mut accumulator = 0.0;
      for (i, obj) in self.light.iter().enumerate() {
        accumulator += self.pdf[i];
        if roulette <= accumulator {
          let sample = obj.geometry.sample();
          return Some(Sample {
            value: sample.value,
            pdf: sample.pdf * self.pdf[i],
          });
        }
      }
      None
    })
  }

  pub fn pdf(&self, geometry: &Box<dyn Geometry + Send + Sync>) -> Option<pdf::Area> {
    self
      .light
      .iter()
      .position(|v| v.geometry.id() == geometry.id())
      .map(|i| geometry.pdf() * self.pdf[i])
  }
}
