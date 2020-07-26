use super::Material;
use math::*;
use sample::*;
use sampler::Sampler;
use util::*;

pub struct Lambertian {
  pub emittance: Vector3,
  pub albedo: Vector3,
}

impl Material for Lambertian {
  fn emittance(&self) -> Vector3 {
    self.emittance
  }

  fn brdf(
    &self,
    _wo: Vector3,
    _wi: Vector3,
    _n: Vector3,
    _x: Vector3,
    _in_to_out: bool,
  ) -> Vector3 {
    // BRDFは半球全体に一様に散乱するDiffuse面を考えると ρ / π
    self.albedo / PI
  }

  fn sample(&self, _wi: Vector3, n: Vector3, _in_to_out: bool) -> Sample<Vector3, pdf::SolidAngle> {
    // 反射点での法線方向を基準にした正規直交基底を生成
    let w = n;
    let basis = w.orthonormal_basis();
    // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
    // (cosにしたがって重点的にサンプル)
    let sample = Sampler::hemisphere_cos_importance();
    let wo = &basis * sample;
    // cos項
    let cos_term = wo.dot(n);
    // 確率密度関数
    // (cosにしたがって重点的にサンプル) cosθ / π
    let pdf = cos_term / PI;
    Sample {
      value: wo,
      pdf: pdf::SolidAngle(pdf),
    }
  }

  fn pdf(&self, _wi: Vector3, wo: Vector3, n: Vector3, _in_to_out: bool) -> pdf::SolidAngle {
    // cos項
    let cos_term = wo.dot(n);
    debug_assert!(wo.is_finite(), "{}", wo);
    debug_assert!(n.is_finite(), "{}", n);
    // 確率密度関数
    // (cosにしたがって重点的にサンプル) cosθ / π
    let pdf = if cos_term > 0.0 { cos_term / PI } else { 0.0 };
    debug_assert!(pdf.is_finite(), "{}", pdf);
    pdf::SolidAngle(pdf)
  }
}
