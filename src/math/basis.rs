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

pub trait BarycentricCoordinate {
  fn barycentric_coordinate(&self, p0: Vector3, p1: Vector3, p2: Vector3) -> Vector3;
}

impl BarycentricCoordinate for Vector3 {
  fn barycentric_coordinate(&self, p0: Vector3, p1: Vector3, p2: Vector3) -> Vector3 {
    let d1 = p0 - p2;
    let d2 = p1 - p2;
    let d = *self - p2;
    let d1x = d1.dot(d1);
    let d2x = d1.dot(d2);
    let d1y = d2x;
    let d2y = d2.dot(d2);
    let dx = d.dot(d1);
    let dy = d.dot(d2);
    let det = d1x * d2y - d1y * d2x;
    let l1 = (dx * d2y - dy * d2x) / det;
    let l2 = (d1x * d2y - dy * d2x) / det;
    let l3 = 1.0 - l1 - l2;
    Vector3::new(l1, l2, l3)
  }
}
