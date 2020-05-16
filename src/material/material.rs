use math::*;
use sample::*;

pub trait Material {
  // 物体自体の放射成分
  fn emission(&self) -> Vector3;
  // 出射ベクトル, 入射ベクトル, 法線ベクトル, 座標 -> BRDF
  fn brdf(&self, Vector3, Vector3, Vector3, Vector3) -> Vector3;
  // 出射ベクトル, 法線ベクトル -> 入射ベクトル, 確率密度
  fn sample(&self, Vector3, Vector3) -> Sample<Vector3, pdf::SolidAngle>;
}
