use super::{Cross, Normalize, Vector3, EPS};

pub trait OrthonormalBasis: Sized {
  fn orthonormal_basis(&self) -> (Self, Self);
}

impl OrthonormalBasis for Vector3 {
  // 自身(normal)を基準として正規直交基底を生成 (正規化済み前提)
  fn orthonormal_basis(&self) -> (Vector3, Vector3) {
    let tangent = if self.x.abs() > EPS {
      Vector3::new(0.0, 1.0, 0.0)
    } else {
      Vector3::new(1.0, 0.0, 0.0)
    }
    .cross(*self)
    .normalize();
    let bionrmal = self.cross(tangent);
    (tangent, bionrmal)
  }
}