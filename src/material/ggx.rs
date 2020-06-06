use super::Material;
use math::*;
use sample::*;

pub struct GGX {
  // 反射率
  pub reflectance: Vector3,
  // 屈折率
  pub ior: f32,
  // ラフネス
  pub roughness: f32,
}

impl GGX {
  fn alpha(&self) -> f32 {
    self.roughness * self.roughness
  }

  fn gaf_smith(&self, wo: Vector3, wi: Vector3, n: Vector3) -> f32 {
    self.g_ggx(wi, n) * self.g_ggx(wo, n)
  }

  fn g_ggx(&self, v: Vector3, n: Vector3) -> f32 {
    let a2 = self.alpha() * self.alpha();
    let cos = v.dot(n);
    let tan = 1.0 / (cos * cos) - 1.0;
    2.0 / (1.0 + (1.0 + a2 * tan * tan).sqrt())
  }

  fn ndf(&self, m: Vector3, n: Vector3) -> f32 {
    let a2 = self.alpha() * self.alpha();
    let mdn = m.dot(n);
    let x = (a2 - 1.0) * mdn * mdn + 1.0;
    a2 / (PI * x * x)
  }

  fn fresnel_schlick(&self, wi: Vector3, m: Vector3) -> f32 {
    let nnn = 1.0 - self.ior;
    let nnp = 1.0 + self.ior;
    let f_0 = (nnn * nnn) / (nnp * nnp);
    let c = wi.dot(m);
    f_0 + (1.0 - f_0) * (1.0 - c).powi(5)
  }
}

impl Material for GGX {
  fn emittance(&self) -> Vector3 {
    Vector3::zero()
  }

  fn brdf(&self, wo: Vector3, wi: Vector3, n: Vector3, _x: Vector3) -> Vector3 {
    if wi.dot(n) <= 0.0 {
      return Vector3::zero();
    }
    debug_assert!(wo.dot(n) > 0.0, "o.n  = {}", wo.dot(n));
    // ハーフベクトル
    let h = (wi + wo).normalize();
    // Torrance-Sparrow model
    let f = self.fresnel_schlick(wi, h);
    debug_assert!(f >= 0.0 && f <= 1.0 && f.is_finite(), "f: {}", f);
    let g = self.gaf_smith(wo, wi, n);
    debug_assert!(g >= 0.0 && g <= 1.0 && g.is_finite(), "g: {}", g);
    let d = self.ndf(h, n);
    debug_assert!(d >= 0.0 && d.is_finite(), "d: {}", d);
    self.reflectance * f * g * d / (4.0 * wi.dot(n) * wo.dot(n))
  }

  fn sample(&self, wo: Vector3, n: Vector3) -> Sample<Vector3, pdf::SolidAngle> {
    // 法線方向を基準にした正規直交基底を生成
    let basis = n.orthonormal_basis();
    // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
    // (brdfの分布にしたがって重点的にサンプル)
    let r1 = 2.0 * PI * rand::random::<f32>();
    let r2 = rand::random::<f32>();
    let tan = self.alpha() * (r2 / (1.0 - r2)).sqrt();
    let x = 1.0 + tan * tan;
    let cos = 1.0 / x.sqrt();
    let sin = tan / x.sqrt();
    // ハーフベクトルをサンプリング
    let h = &basis * Vector3::new(r1.cos() * sin, r1.sin() * sin, cos);
    // 入射ベクトル
    let o_h = wo.dot(h);
    let in_ = h * (2.0 * o_h) - wo;
    // ヤコビアン
    let jacobian = 1.0 / (4.0 * o_h);
    // 確率密度関数
    let pdf = self.ndf(h, n) * h.dot(n) * jacobian;
    Sample {
      value: in_,
      pdf: pdf::SolidAngle(pdf),
    }
  }

  fn pdf(&self, wo: Vector3, wi: Vector3, n: Vector3) -> pdf::SolidAngle {
    // ハーフベクトル
    let h = ((wi + wo) / 2.0).normalize();
    let o_h = wo.dot(h);
    // ヤコビアン
    let jacobian = 1.0 / (4.0 * o_h);
    // 確率密度関数
    let pdf = self.ndf(h, n) * h.dot(n) * jacobian;
    pdf::SolidAngle(pdf)
  }
}
