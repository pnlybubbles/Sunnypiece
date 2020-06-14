use geometry::Geometry;
use math::*;
use object::Object;
use rand::Rng;
use sample::{pdf, Sample};
use RNG;

pub struct LightSampler<'a> {
  light: Vec<&'a Object<'a>>,
}

impl<'a> LightSampler<'a> {
  pub fn new(objects: &'a Vec<Object>) -> Self {
    // 光源だけ取り出す
    let light = objects
      .iter()
      .filter(|v| v.material.emittance().sqr_norm() > 0.0)
      .collect::<Vec<_>>();
    LightSampler { light: light }
  }

  fn pdf_map(&self, x: Vector3, n: Vector3) -> Option<Vec<f32>> {
    // 光源サンプリングのCDFマップを計算
    let intensity = self
      .light
      .iter()
      .map(|v| {
        let path = v.geometry.aabb().center - x;
        let wi = path.normalize();
        let wo = -wi;
        let n2 = v.geometry.normal(x);
        v.geometry.area()
          * v.material.emittance().max()
          * special::chi(wi.dot(n))
          * special::chi(wo.dot(n2))
          / path.sqr_norm()
      })
      .collect::<Vec<_>>();

    let intensity_sum: f32 = intensity.iter().sum();

    if intensity_sum < EPS {
      None
    } else {
      Some(
        intensity
          .iter()
          .map(|v| v / intensity_sum)
          .collect::<Vec<_>>(),
      )
    }
  }

  /**
   * 光源の重点的サンプリング
   *
   * NOTE: 位置ベクトルがサンプリングされる
   */
  pub fn sample(&self, x: Vector3, n: Vector3) -> Option<Sample<Vector3, pdf::Area>> {
    self.pdf_map(x, n).and_then(|pdf_map| {
      RNG.with(|rng| {
        let roulette = rng.borrow_mut().gen::<f32>();
        let mut accumulator = 0.0;
        for (i, obj) in self.light.iter().enumerate() {
          accumulator += pdf_map[i];
          if roulette <= accumulator {
            let sample = obj.geometry.sample();
            return Some(Sample {
              value: sample.value,
              pdf: sample.pdf * pdf_map[i],
            });
          }
        }
        None
      })
    })
  }

  pub fn pdf(
    &self,
    geometry: &Box<dyn Geometry + Send + Sync>,
    x: Vector3,
    n: Vector3,
  ) -> Option<pdf::Area> {
    let i = self
      .light
      .iter()
      .position(|v| v.geometry.id() == geometry.id())
      .expect("ERROR: geometry must be a light source!");
    self
      .pdf_map(x, n)
      .map(|pdf_map| geometry.pdf() * pdf_map[i])
  }
}
