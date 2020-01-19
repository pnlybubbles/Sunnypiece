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
