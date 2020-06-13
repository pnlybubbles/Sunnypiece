use super::num::Zero;
use super::vector::{Cross, Dot, Map};
use super::vector4::Vector4;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Vector3 { x: x, y: y, z: z }
  }

  pub fn fill(v: f32) -> Self {
    Vector3 { x: v, y: v, z: v }
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

impl Index<usize> for Vector3 {
  type Output = f32;

  fn index(&self, i: usize) -> &f32 {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Vector3 index out of bounds."),
    }
  }
}

impl From<Vector4> for Vector3 {
  fn from(v: Vector4) -> Vector3 {
    Vector3 {
      x: v.x,
      y: v.y,
      z: v.z,
    }
  }
}

impl From<[f32; 3]> for Vector3 {
  fn from(v: [f32; 3]) -> Vector3 {
    Vector3 {
      x: v[0],
      y: v[1],
      z: v[2],
    }
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

impl Map<f32> for Vector3 {
  fn map<F: Fn(f32) -> f32>(self, f: F) -> Vector3 {
    Vector3::new(f(self.x), f(self.y), f(self.z))
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

impl AddAssign for Vector3 {
  fn add_assign(&mut self, rhs: Vector3) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  fn sub(self, rhs: Vector3) -> Vector3 {
    Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl SubAssign for Vector3 {
  fn sub_assign(&mut self, rhs: Vector3) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;

  fn mul(self, rhs: f32) -> Vector3 {
    Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl MulAssign<f32> for Vector3 {
  fn mul_assign(&mut self, rhs: f32) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
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

impl DivAssign<f32> for Vector3 {
  fn div_assign(&mut self, rhs: f32) {
    self.x /= rhs;
    self.y /= rhs;
    self.z /= rhs;
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
