use math::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
  pub origin: Vector3,
  pub direction: Vector3,
}
