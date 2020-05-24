use math::*;
use sample::*;

pub trait Material {
  // 物体自体の放射成分
  fn emittance(&self) -> Vector3;
  // 出射ベクトル, 入射ベクトル, 法線ベクトル, 座標 -> BRDF
  fn brdf(&self, wo: Vector3, wi: Vector3, n: Vector3, x: Vector3) -> Vector3;
  // 出射ベクトル, 法線ベクトル -> 入射ベクトル, 確率密度
  fn sample(&self, wo: Vector3, n: Vector3) -> Sample<Vector3, pdf::SolidAngle>;
  // 入射ベクトル, 法線ベクトル -> 確率密度
  fn pdf(&self, wi: Vector3, n: Vector3) -> pdf::SolidAngle;
}
