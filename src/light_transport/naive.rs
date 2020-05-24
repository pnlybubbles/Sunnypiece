use super::radiance::Radiance;
use acceleration::Acceleration;
use math::*;
use object::{GeomWeight, Interaction};
use ray::Ray;

pub struct Naive<'a, S>
where
  S: Acceleration,
{
  structure: &'a S,
}

impl<'a, S> Naive<'a, S>
where
  S: Acceleration + 'a,
{
  pub fn new(structure: &'a S) -> Self {
    Naive {
      structure: structure,
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
      Some(geom) => {
        let li = self.radiance_recursive(&geom.next, depth + 1);
        li * geom.bsdf() * geom.weight(material_sample.pdf)
      }
    };

    le + material_throughput
  }
}

impl<'a, S> Radiance for Naive<'a, S>
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
