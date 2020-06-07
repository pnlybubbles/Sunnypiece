use math::*;
use std::ops::Mul;

pub trait Measure {}

#[derive(Copy, Clone)]
pub struct SolidAngle(pub f32);
#[derive(Copy, Clone)]
pub struct Area(pub f32);

impl Measure for SolidAngle {}
impl Measure for Area {}

impl Mul<f32> for SolidAngle {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    let SolidAngle(lhs) = self;
    SolidAngle(rhs * lhs)
  }
}

impl Mul<f32> for Area {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    let Area(lhs) = self;
    Area(rhs * lhs)
  }
}

impl SolidAngle {
  pub fn area_measure(self, x: Vector3, x2: Vector3, n2: Vector3) -> Area {
    let SolidAngle(pdf) = self;
    let path = x2 - x;
    let wo = path.normalize();
    debug_assert!(
      (pdf * (-wo).dot(n2) / path.sqr_norm()).is_finite(),
      "{} {} {} {} {}",
      pdf,
      path,
      wo,
      (-wo).dot(n2),
      path.sqr_norm()
    );
    Area(pdf * (-wo).dot(n2) / path.sqr_norm())
  }
}
