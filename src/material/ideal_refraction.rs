use super::physics::*;
use super::Material;
use math::*;
use sample::*;
use sampler::Roulette;

pub struct IdealRefraction {
  // 反射率
  pub reflectance: Vector3,
  // 屈折率
  pub ior: f32,
}

impl IdealRefraction {
  fn ior(&self, in_to_out: bool) -> (f32, f32) {
    // (入射媒質屈折率, 出射媒質屈折率)
    if in_to_out {
      (self.ior, 1.0)
    } else {
      (1.0, self.ior)
    }
  }
}

impl Material for IdealRefraction {
  fn emittance(&self) -> Vector3 {
    Vector3::zero()
  }

  fn brdf(&self, wi: Vector3, wo: Vector3, n: Vector3, _x: Vector3, in_to_out: bool) -> Vector3 {
    let (ni, no) = self.ior(in_to_out);
    let coef = wi
      // 屈折
      .refract(n, ni / no)
      // フレネル反射
      .map(|wt| {
        let f = Fresnel::ior(wi, wt, n, ni, no);
        if wo.dot(n) > 0.0 {
          // 反射
          f / wo.dot(n)
        } else {
          // 透過
          (1.0 - f) * (no / ni).powi(2) / -wo.dot(n)
        }
      })
      // 全反射
      .unwrap_or(1.0 / wo.dot(n));
    self.reflectance * distribution::DELTA_FUNCTION * coef
  }

  fn sample(&self, wi: Vector3, n: Vector3, in_to_out: bool) -> Sample<Vector3, pdf::SolidAngle> {
    debug_assert!(wi.dot(n) > 0.0);
    let (ni, no) = self.ior(in_to_out);
    let wo = wi
      // 屈折
      .refract(n, ni / no)
      // フレネル反射
      .map(|wt| {
        let f = Fresnel::ior(wi, wt, n, ni, no);
        // ロシアンルーレットで分岐
        if Roulette::within(f) {
          wi.reflect(n)
        } else {
          wt
        }
      })
      // 全反射
      .unwrap_or(wi.reflect(n));
    // 確率密度関数
    let pdf = self.pdf(wi, wo, n, in_to_out);
    Sample {
      value: wo,
      pdf: pdf,
    }
  }

  fn pdf(&self, wi: Vector3, wo: Vector3, n: Vector3, in_to_out: bool) -> pdf::SolidAngle {
    let (ni, no) = self.ior(in_to_out);
    let p = wi
      .refract(n, ni / no)
      .map(|wt| {
        let f = Fresnel::ior(wi, wt, n, ni, no);
        if wo.dot(n) > 0.0 {
          f
        } else {
          1.0 - f
        }
      })
      .unwrap_or(1.0);
    return pdf::SolidAngle(distribution::DELTA_FUNCTION * p);
  }

  fn is_delta(&self) -> bool {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ior_test() {
    let ior = 1.5;
    let mat = IdealRefraction {
      reflectance: Vector3::new(1.0, 1.0, 1.0),
      ior,
    };
    let (ni, no) = mat.ior(false);
    assert!(ni == 1.0);
    assert!(no == ior);
    let (ni, no) = mat.ior(true);
    assert!(ni == ior);
    assert!(no == 1.0);
  }
}
