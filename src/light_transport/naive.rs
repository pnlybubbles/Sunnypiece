use super::radiance::Radiance;
use acceleration::Acceleration;
use camera::Camera;
use math::*;
use ray::Ray;

pub struct Naive<T, S>
where
  T: Camera,
  S: Acceleration,
{
  pub camera: T,
  pub structure: S,
}

impl<T, S> Radiance for Naive<T, S>
where
  T: Camera,
  S: Acceleration,
{
  fn radiance(&self, ray: &Ray) -> Vector3 {
    let maybe_intersection = self.structure.intersect(ray);

    match maybe_intersection {
      None => Vector3::zero(),
      Some(intersection) => Vector3::new(1.0, 1.0, 1.0),
    }
  }
}
