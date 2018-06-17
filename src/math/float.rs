use super::traits::ApproxEq;
use super::constant::EPS;

impl ApproxEq for f32 {
  fn approx_eq(self, other: f32) -> bool {
    (self - other).abs() < EPS
  }
}
