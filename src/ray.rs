use math::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
  // Geometry id
  pub from: Option<usize>,
  pub origin: Vector3,
  pub direction: Vector3,
}
