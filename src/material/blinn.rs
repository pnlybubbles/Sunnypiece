use super::physics::*;
use super::Material;
use math::*;
use sample::*;

pub struct Blinn {
  // 反射率
  pub reflectance: Vector3,
  // ラフネス
  pub roughness: f32,
}

impl Material for Blinn {
  fn emittance(&self) -> Vector3 {
    Vector3::zero()
  }

  fn brdf(&self, wi: Vector3, wo: Vector3, n: Vector3, _x: Vector3) -> Vector3 {
    // ハーフベクトル
    let wh = (wo + wi).normalize();
    let a = self.roughness;
    self.reflectance * ((a + 2.0) / (8.0 * PI) * n.dot(wh).powf(a))
  }

  fn sample(&self, wi: Vector3, n: Vector3) -> Sample<Vector3, pdf::SolidAngle> {
    // 法線方向を基準にした正規直交基底を生成
    let basis = n.orthonormal_basis();
    // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
    // (brdfの分布にしたがって重点的にサンプル)
    let cos = rand::random::<f32>().powf(1.0 / self.roughness);
    let phi = 2.0 * PI * rand::random::<f32>();
    let sin = (1.0 - cos * cos).sqrt();
    // ハーフベクトルをサンプリング
    let wh = &basis * Vector3::new(phi.cos() * sin, phi.sin() * sin, cos);
    // 入射ベクトル
    let wo = wi.reflect(wh);
    // 確率密度関数
    let pdf = self.pdf(wi, wo, n);
    Sample {
      value: wo,
      pdf: pdf,
    }
  }

  fn pdf(&self, wi: Vector3, wo: Vector3, n: Vector3) -> pdf::SolidAngle {
    // ハーフベクトル
    let wh = (wo + wi).normalize();
    let a = self.roughness;
    // 正規化項
    let nf = (a + 1.0) / (2.0 * PI);
    // 確率密度関数
    let pdf = n.dot(wh).powf(a) * nf / (4.0 * wi.dot(wh));
    pdf::SolidAngle(pdf)
  }
}
