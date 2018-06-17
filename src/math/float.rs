use super::num::Zero;
use super::vector::Dot;

impl Dot for f32 {
  fn dot(self, rhs: f32) -> f32 {
    self * rhs
  }
}

impl Zero for f32 {
  fn zero() -> f32 {
    0.0
  }
}
