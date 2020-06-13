use super::*;

pub trait OrthonormalBasis: Sized {
  fn orthonormal_basis(&self) -> Matrix4;
}

impl OrthonormalBasis for Vector3 {
  // 自身(normal)を基準として正規直交基底を生成 (正規化済み前提)
  // Gram Schmidtの正規直交化法
  fn orthonormal_basis(&self) -> Matrix4 {
    // If n is near the x-axis , use the y- axis . Otherwise use the x- axis .
    let n = *self;
    let mut b1 = if n.x.abs() > 0.9 {
      Vector3::new(0.0, 1.0, 0.0)
    } else {
      Vector3::new(1.0, 0.0, 0.0)
    };
    b1 -= n * b1.dot(n); // Make b1 orthogonal to n
    b1 *= 1.0 / b1.dot(b1).sqrt(); // Normalize b1
    let b2 = n.cross(b1); // Construct b2 using a cross product
    [b1, b2, n].into()
  }
}
