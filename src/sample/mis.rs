use super::pdf::{Area, Measure, SolidAngle};

pub trait MIS: Measure {
  fn balance_hulistic(self, Self) -> f32;
  fn power_hulistic(self, Self, b: i32) -> f32;
}

impl MIS for SolidAngle {
  fn balance_hulistic(self, rhs: Self) -> f32 {
    let SolidAngle(p1) = self;
    let SolidAngle(p2) = rhs;
    mis_balance_hulistic(p1, p2)
  }

  fn power_hulistic(self, rhs: Self, b: i32) -> f32 {
    let SolidAngle(p1) = self;
    let SolidAngle(p2) = rhs;
    mis_power_hulistic(p1, p2, b)
  }
}

impl MIS for Area {
  fn balance_hulistic(self, rhs: Self) -> f32 {
    let Area(p1) = self;
    let Area(p2) = rhs;
    mis_balance_hulistic(p1, p2)
  }

  fn power_hulistic(self, rhs: Self, b: i32) -> f32 {
    let Area(p1) = self;
    let Area(p2) = rhs;
    mis_power_hulistic(p1, p2, b)
  }
}

fn mis_balance_hulistic(p1: f32, p2: f32) -> f32 {
  p1 / (p1 + p2)
}

fn mis_power_hulistic(p1: f32, p2: f32, b: i32) -> f32 {
  let p1b = p1.powi(b);
  let p2b = p2.powi(b);
  p1b / (p1b + p2b)
}
