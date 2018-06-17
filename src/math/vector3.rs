use std::fmt;
use std::ops::{Neg, Add, Sub, Mul, Div};
use super::traits::{Zero, Dot, Cross, ApproxEq};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x: x, y: y, z: z }
  }
}

impl ApproxEq for Vector3 {
  fn approx_eq(self, other: Vector3) -> bool {
    self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
  }
}

impl Zero for Vector3 {
  fn zero() -> Vector3 {
    Vector3::new(0.0, 0.0, 0.0)
  }
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl Dot for Vector3 {
  fn dot(self, rhs: Vector3) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }
}

impl Cross for Vector3 {
  fn cross(self, rhs: Vector3) -> Vector3 {
    Vector3::new(
      self.y * rhs.z - self.z * rhs.y,
      self.z * rhs.x - self.x * rhs.z,
      self.x * rhs.y - self.y * rhs.x,
    )
  }
}

impl Neg for Vector3 {
  type Output = Vector3;

  fn neg(self) -> Vector3 {
    Vector3::new(-self.x, -self.y, -self.z)
  }
}

impl Add for Vector3 {
  type Output = Vector3;

  fn add(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  fn sub(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;

  fn mul(self, rhs: f32) -> Vector3 {
    Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl Mul<Vector3> for f32 {
  type Output = Vector3;

  fn mul(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
  }
}

impl Mul for Vector3 {
  type Output = Vector3;

  fn mul(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl Div<f32> for Vector3 {
  type Output = Vector3;

  fn div(self, rhs: f32) -> Vector3 {
    Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl Div<Vector3> for f32 {
  type Output = Vector3;

  fn div(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self / rhs.x, self / rhs.y, self / rhs.z)
  }
}

impl Div for Vector3 {
  type Output = Vector3;

  fn div(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
  }
}

#[cfg(test)]
mod tests {
  use Vector3;
  use math::traits::ApproxEq;

  #[quickcheck]
  fn new(x: f32, y: f32, z: f32) -> bool {
    let v = Vector3::new(x, y, z);
    v.x == x && v.y == y && v.z == z
  }

  #[quickcheck]
  fn add(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let v1 = Vector3::new(x1, y1, z1);
    let v2 = Vector3::new(x2, y2, z2);
    let r = v1 + v2;
    r.x == x1 + x2 && r.y == y1 + y2 && r.z == z1 + z2
  }

  #[quickcheck]
  fn add_commute(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let v1 = Vector3::new(x1, y1, z1);
    let v2 = Vector3::new(x2, y2, z2);
    let r1 = v1 + v2;
    let r2 = v2 + v1;
    r1 == r2
  }

  #[quickcheck]
  fn neg(x: f32, y: f32, z: f32) -> bool {
    let v = -Vector3::new(x, y, z);
    v.x == -x && v.y == -y && v.z == -z
  }

  #[quickcheck]
  fn sub(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let v1 = Vector3::new(x1, y1, z1);
    let v2 = Vector3::new(x2, y2, z2);
    let r1 = v1 - v2;
    let r2 = v1 + (-v2);
    r1 == r2
  }

  #[quickcheck]
  fn mul(x: f32, y: f32, z: f32, s: f32) -> bool {
    let v = Vector3::new(x, y, z) * s;
    v.x == x * s && v.y == y * s && v.z == z * s
  }
  
  #[quickcheck]
  fn mul_commute(x: f32, y: f32, z: f32, s: f32) -> bool {
    let v = Vector3::new(x, y, z);
    let r1 = v * s;
    let r2 = s * v;
    r1 == r2
  }

  #[quickcheck]
  fn div(x: f32, y: f32, z: f32, s: f32) -> bool {
    if s == 0.0 { return true }
    let v = Vector3::new(x, y, z);
    let r1 = v / s;
    let r2 = v * (1.0 / s);
    r1.approx_eq(r2)
  }

  #[quickcheck]
  fn div_inverse(x: f32, y: f32, z: f32, s: f32) -> bool {
    if x == 0.0 || y == 0.0 || z == 0.0 { return true }
    let v = Vector3::new(x, y, z);
    let r = s / v;
    r.x == s / x && r.y == s / y && r.z == s / z
  }

  #[quickcheck]
  fn dot(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let v1 = Vector3::new(x1, y1, z1);
    let v2 = Vector3::new(x2, y2, z2);
    let r = v1.dot(v2);
    r == x1 * x2 + y1 * y2 + z1 * z2
  }
}
