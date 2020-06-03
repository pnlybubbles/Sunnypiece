use math::*;

impl<T> IsNormalized for T
where
  T: Norm,
{
  fn is_normalized(self) -> bool {
    return self.norm().approx_eq(1.0);
  }
}

pub trait IsNormalized {
  fn is_normalized(self) -> bool;
}

impl<T> LessThanUnit for T
where
  T: Norm,
{
  fn less_than_unit(self) -> bool {
    let norm = self.norm();
    return norm >= 0.0 && norm < 1.0;
  }
}

pub trait LessThanUnit {
  fn less_than_unit(self) -> bool;
}

impl ToColor for Vector3 {
  fn to_color(self) -> Vector3 {
    return self / 2.0 + Vector3::new(0.5, 0.5, 0.5);
  }
}

pub trait ToColor {
  fn to_color(self) -> Vector3;
}

pub trait Finite {
  fn is_finite(self) -> bool;
  fn is_nan(self) -> bool;
  fn is_infinite(self) -> bool;
}

impl Finite for Vector3 {
  fn is_finite(self) -> bool {
    self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
  }

  fn is_nan(self) -> bool {
    self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
  }

  fn is_infinite(self) -> bool {
    self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite()
  }
}
