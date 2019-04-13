use std::fmt;
use super::vector::{Dot};
use super::vector3::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Vector4 {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
    Vector4 { x: x, y: y, z: z, w: w }
  }
}

impl fmt::Display for Vector4 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
  }
}

impl From<Vector3> for Vector4 {
  fn from(v: Vector3) -> Vector4 {
    Vector4 { x: v.x, y: v.y, z: v.z, w: 1.0 }
  }
}

impl Into<[f32; 4]> for Vector4 {
  fn into(self) -> [f32; 4] {
    [self.x, self.y, self.z, self.w]
  }
}

impl From<[f32; 4]> for Vector4 {
  fn from(v: [f32; 4]) -> Vector4 {
    Vector4::new(v[0], v[1], v[2], v[3])
  }
}

impl Dot for Vector4 {
  fn dot(self, rhs: Vector4) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
  }
}
