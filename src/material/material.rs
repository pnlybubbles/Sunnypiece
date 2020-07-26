use math::*;
use sample::*;

pub trait Material {
  // 物体自体の放射成分
  fn emittance(&self) -> Vector3;
  // 入射ベクトル, 出射ベクトル, 法線ベクトル, 座標 -> BRDF
  fn brdf(&self, wi: Vector3, wo: Vector3, n: Vector3, x: Vector3, in_to_out: bool) -> Vector3;
  // 入射ベクトル, 法線ベクトル -> 出射ベクトル, 確率密度
  fn sample(&self, wi: Vector3, n: Vector3, in_to_out: bool) -> Sample<Vector3, pdf::SolidAngle>;
  // 入射ベクトル, 出射ベクトル, 法線ベクトル -> 確率密度
  fn pdf(&self, wi: Vector3, wo: Vector3, n: Vector3, in_to_out: bool) -> pdf::SolidAngle;
  // 確率密度関数がDirac-Delta関数に依存するかどうか
  fn is_delta(&self) -> bool {
    false
  }
}
