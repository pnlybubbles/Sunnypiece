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
  fn radiance(&self, ray: Ray) -> Vector3 {
    debug_assert!(
      ray.direction.is_normalized(),
      "ray direction should be normalized."
    );

    let maybe_interaction = self.structure.interact(ray);

    match maybe_interaction {
      None => Vector3::zero(),
      Some(interaction) => interaction.normal().to_color(),
    }
  }
}
