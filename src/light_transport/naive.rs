use super::radiance::Radiance;
use acceleration::Acceleration;
use math::*;
use ray::Ray;

pub struct Naive<S>
where
  S: Acceleration,
{
  pub structure: S,
}

impl<S> Naive<S>
where
  S: Acceleration,
{
  fn radiance_recursive(&self, ray: &Ray, depth: usize) -> Vector3 {
    let maybe_interaction = self.structure.interact(ray);

    match maybe_interaction {
      None => Vector3::zero(),
      Some(interaction) => {
        if interaction.is_backface() {
          return Vector3::zero();
        }
        let l_e = interaction.emittance();
        if depth > 5 {
          return l_e;
        }
        let (new_ray, throughput) = interaction.material_throughput();
        let l_i = self.radiance_recursive(&new_ray, depth + 1);
        l_e + l_i * throughput
      }
    }
  }
}

impl<S> Radiance for Naive<S>
where
  S: Acceleration,
{
  fn radiance(&self, ray: &Ray) -> Vector3 {
    self.radiance_recursive(ray, 0)
  }
}
