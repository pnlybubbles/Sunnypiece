use super::radiance::Radiance;
use acceleration::Acceleration;
use math::*;
use ray::Ray;
use util::{IsNormalized, ToColor};

pub struct Normal<S>
where
  S: Acceleration,
{
  pub structure: S,
}

impl<S> Radiance for Normal<S>
where
  S: Acceleration,
{
  fn radiance(&self, ray: &Ray) -> Vector3 {
    debug_assert!(
      ray.direction.is_normalized(),
      "ray direction should be normalized."
    );

    let maybe_intersection = self.structure.intersect(ray);

    match maybe_intersection {
      None => Vector3::zero(),
      Some(intersection) => intersection.normal.to_color(),
    }
  }
}
