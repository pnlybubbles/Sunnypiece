use math::*;
use rand::Rng;
use RNG;

pub struct Sampler;

impl Sampler {
  pub fn hemisphere_cos_importance() -> Vector3 {
    RNG.with(|rng| {
      // 乱数を生成
      // (cosにしたがって重点的にサンプル)
      let r1 = 2.0 * PI * rng.borrow_mut().gen::<f32>();
      let r2 = rng.borrow_mut().gen::<f32>();
      let r2s = r2.sqrt();
      // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
      // (cosにしたがって重点的にサンプル)
      Vector3::new(r1.cos() * r2s, r1.sin() * r2s, (1.0 - r2).sqrt())
    })
  }

  pub fn hemisphere_uniform() -> Vector3 {
    RNG.with(|rng| {
      // 乱数を生成
      let r1 = 2.0 * PI * rng.borrow_mut().gen::<f32>();
      let r2 = rng.borrow_mut().gen::<f32>();
      let r2s = (1.0 - r2 * r2).sqrt();
      // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
      // (一様サンプル)
      Vector3::new(r1.cos() * r2s, r1.sin() * r2s, r2.sqrt())
    })
  }

  pub fn sphere_uniform() -> Vector3 {
    RNG.with(|rng| {
      // 乱数を生成
      let r1 = 2.0 * PI * rng.borrow_mut().gen::<f32>();
      let r2 = rng.borrow_mut().gen::<f32>() * 2.0 - 1.0;
      let r2s = (1.0 - r2 * r2).sqrt();
      // 球面極座標を用いて反射点から単位半球面上のある一点へのベクトルを生成
      // (一様サンプル)
      Vector3::new(r1.cos() * r2s, r1.sin() * r2s, r2)
    })
  }
}
