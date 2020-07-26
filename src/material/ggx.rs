use super::physics::*;
use super::Material;
use math::*;
use sample::*;
use util::Finite;

pub struct GGX {
  // 反射率
  pub reflectance: Vector3,
  // ラフネス
  pub roughness: f32,
}

impl GGX {
  fn alpha(&self) -> f32 {
    self.roughness * self.roughness
  }

  fn g_ggx(&self, wi: Vector3, wo: Vector3, n: Vector3) -> f32 {
    self.g1(wo, n) * self.g1(wi, n)
  }

  fn g1(&self, v: Vector3, n: Vector3) -> f32 {
    let a2 = self.alpha() * self.alpha();
    let cos = v.dot(n);
    let tan2 = 1.0 / cos.powi(2) - 1.0;
    special::chi(cos) * 2.0 / (1.0 + (1.0 + a2 * tan2).sqrt())
  }

  fn d_ggx(&self, m: Vector3, n: Vector3) -> f32 {
    let a2 = self.alpha().powi(2);
    a2 / (PI * ((a2 - 1.0) * m.dot(n).powi(2) + 1.0).powi(2))
  }
}

impl Material for GGX {
  fn emittance(&self) -> Vector3 {
    Vector3::zero()
  }

  fn brdf(&self, wi: Vector3, wo: Vector3, n: Vector3, _x: Vector3, _in_to_out: bool) -> Vector3 {
    // ハーフベクトル
    let wh = (wo + wi).normalize();
    // Torrance-Sparrow model
    let f = Fresnel::schlick(self.reflectance, wo, wh);
    debug_assert!(f.x > 0.0 && f.is_finite(), "f: {}", f);
    let g = self.g_ggx(wi, wo, n);
    debug_assert!(g >= 0.0 && g <= 1.0 && g.is_finite(), "g: {}", g);
    let d = self.d_ggx(wh, n);
    debug_assert!(d >= 0.0 && d.is_finite(), "d: {}", d);
    f * (g * d / (4.0 * wi.dot(n) * wo.dot(n)))
  }

  fn sample(&self, wi: Vector3, n: Vector3, in_to_out: bool) -> Sample<Vector3, pdf::SolidAngle> {
    // 法線方向を基準にした正規直交基底を生成
    let basis = n.orthonormal_basis();
    // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
    // (brdfの分布にしたがって重点的にサンプル)
    let r1 = 2.0 * PI * rand::random::<f32>();
    let r2 = rand::random::<f32>();
    let tan = self.alpha() * r2.sqrt() / (1.0 - r2).sqrt();
    let cos = 1.0 / (1.0 + tan * tan).sqrt();
    let sin = tan * cos;
    // ハーフベクトルをサンプリング
    let wh = &basis * Vector3::new(r1.cos() * sin, r1.sin() * sin, cos);
    // 入射ベクトル
    let wo = wi.reflect(wh);
    // 確率密度関数
    let pdf = self.pdf(wi, wo, n, in_to_out);
    Sample {
      value: wo,
      pdf: pdf,
    }
  }

  fn pdf(&self, wi: Vector3, wo: Vector3, n: Vector3, _in_to_out: bool) -> pdf::SolidAngle {
    // ハーフベクトル
    let wh = (wo + wi).normalize();
    // 確率密度関数
    let pdf = self.d_ggx(wh, n) * wh.dot(n) / (4.0 * wo.dot(n) * wi.dot(wh));
    pdf::SolidAngle(pdf)
  }
}
