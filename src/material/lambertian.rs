use super::Material;
use math::*;
use sample::*;
use sampler::Sampler;

pub struct Lambertian {
  pub emittance: Vector3,
  pub albedo: Vector3,
}

impl Material for Lambertian {
  fn emittance(&self) -> Vector3 {
    self.emittance
  }

  fn brdf(&self, _out_: Vector3, _in_: Vector3, _n_: Vector3, _pos: Vector3) -> Vector3 {
    // BRDFは半球全体に一様に散乱するDiffuse面を考えると ρ / π
    self.albedo / PI
  }

  fn sample(&self, _out_: Vector3, n: Vector3) -> Sample<Vector3, pdf::SolidAngle> {
    // 反射点での法線方向を基準にした正規直交基底を生成
    let w = n;
    let (u, v) = w.orthonormal_basis();
    // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
    // (cosにしたがって重点的にサンプル)
    let sample = Sampler::hemisphere_cos_importance();
    let in_ = u * sample.x + v * sample.y + w * sample.z;
    // cos項
    let cos_term = in_.dot(n);
    // 確率密度関数
    // (cosにしたがって重点的にサンプル) cosθ / π
    let pdf = cos_term / PI;
    Sample {
      value: in_,
      pdf: pdf::SolidAngle(pdf),
    }
  }

  fn pdf(&self, wi: Vector3, n: Vector3) -> pdf::SolidAngle {
    // cos項
    let cos_term = wi.dot(n);
    // 確率密度関数
    // (cosにしたがって重点的にサンプル) cosθ / π
    let pdf = cos_term / PI;
    pdf::SolidAngle(pdf)
  }
}
