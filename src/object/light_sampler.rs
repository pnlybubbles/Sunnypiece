use geometry::Geometry;
use math::*;
use object::Object;
use rand::Rng;
use sample::{pdf, Sample};
use RNG;

pub struct LightSampler<'a> {
  light: Vec<&'a Object<'a>>,
  intensity: Vec<f32>,
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
    LightSampler {
      light: light,
      intensity,
    }
  }

  fn pdf_map(&self, x: Vector3, n: Vector3) -> Option<Vec<f32>> {
    // 光源サンプリングのPDFマップを計算
    let pdf_map = self
      .light
      .iter()
      .enumerate()
      .map(|(i, v)| {
        let path = v.geometry.aabb().center - x;
        let path_sqr_norm = path.sqr_norm();
        let wo = path / path_sqr_norm.sqrt();
        let wo_n = wo.dot(n);
        if wo_n < 0.0 {
          return 0.0;
        }
        let n2 = v.geometry.normal(x);
        let wo_n2 = (-wo).dot(n2);
        if wo_n2 < 0.0 {
          return 0.0;
        }
        self.intensity[i] * (wo_n * wo_n2).max(0.2) / path_sqr_norm
      })
      .collect::<Vec<_>>();

    let normalize_factor: f32 = pdf_map.iter().sum();

    if normalize_factor < EPS {
      None
    } else {
      Some(
        pdf_map
          .iter()
          .map(|v| v / normalize_factor)
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
