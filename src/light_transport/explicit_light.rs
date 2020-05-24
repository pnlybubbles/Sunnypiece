use super::radiance::Radiance;
use acceleration::{Acceleration, AccelerationUtility};
use math::*;
use object::{Interaction, LightSampler, RelationWeight};
use ray::Ray;

pub struct ExplicitLight<'a, S>
where
  S: Acceleration,
{
  structure: &'a S,
  light_sampler: LightSampler<'a>,
}

impl<'a, S> ExplicitLight<'a, S>
where
  S: Acceleration + 'a,
{
  pub fn new(structure: &'a S) -> Self {
    ExplicitLight {
      structure: structure,
      light_sampler: structure.light_sampler(),
    }
  }

  fn radiance_recursive(&self, point: &Interaction, depth: usize) -> Vector3 {
    let le = point.emittance();
    // スタックオーバーフロー防止
    // TODO: ロシアンルーレット
    if depth > 5 {
      return le;
    }

    // マテリアルに基づいたサンプリング
    let material_sample = point.sample_material();
    let material_throughput = match point.connect_direction(self.structure, material_sample.value) {
      None => Vector3::zero(),
      Some(relation) => {
        let li = self.radiance_recursive(&relation.next, depth + 1);
        li * relation.bsdf() * relation.weight(material_sample.pdf)
      }
    };

    // 明示的な光源のサンプリング
    let light_sample = self.light_sampler.sample();
    let light_throughput = match point.connect_point(self.structure, light_sample.value) {
      None => Vector3::zero(),
      Some(relation) => {
        let li = relation.next.emittance();
        li * relation.bsdf() * relation.weight(light_sample.pdf)
      }
    };

    le + material_throughput + light_throughput
  }
}

impl<'a, S> Radiance for ExplicitLight<'a, S>
where
  S: Acceleration,
{
  fn radiance(&self, ray: Ray) -> Vector3 {
    let maybe_interaction = self.structure.interact(ray);

    match maybe_interaction {
      None => Vector3::zero(),
      Some(interaction) => self.radiance_recursive(&interaction, 0),
    }
  }
}
