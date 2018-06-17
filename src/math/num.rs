use std::ops::Sub;
use super::vector::Norm;
use super::constant::EPS;

pub trait Zero {
  fn zero() -> Self;
}

pub trait ApproxEq {
  fn approx_eq(self, Self) -> bool;
}

impl<T> ApproxEq for T
  where T: Norm + Sub<Output = T>
{
  fn approx_eq(self, rhs: T) -> bool {
    (self - rhs).sqr_norm() < EPS
  }
}
