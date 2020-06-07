use super::radiance::Radiance;
use acceleration::Acceleration;
use math::*;
use ray::Ray;
use sampler::Sampler;

pub struct IntersectionTest<'a, S>
where
  S: Acceleration,
{
  structure: &'a S,
}

impl<'a, S> IntersectionTest<'a, S>
where
  S: Acceleration + 'a,
{
  pub fn new(structure: &'a S) -> Self {
    IntersectionTest {
      structure: structure,
    }
  }
}

impl<'a, S> Radiance for IntersectionTest<'a, S>
where
  S: Acceleration,
{
  fn radiance(&self, ray: Ray) -> Vector3 {
    let maybe_interaction = self.structure.interact(ray);

    match maybe_interaction {
      None => Vector3::zero(),
      Some(point) => {
        for _ in 0..1000 {
          let sample = Sampler::hemisphere_uniform();
          let basis = point.orienting_normal.orthonormal_basis();
          let direction = &basis * sample;
          match point.connect_direction(self.structure, direction) {
            None => (),
            Some(geom) => {
              if geom.current.orienting_normal.dot(geom.wo) > EPS {
                println!(
                  "{} {} {}",
                  geom.current.orienting_normal.dot(geom.wo),
                  geom.next.orienting_normal.dot(-geom.wo),
                  geom.path_i.norm(),
                );
                return Vector3::new(1.0, 0.0, 0.0);
              }
            }
          }
        }
        return Vector3::new(0.0, 1.0, 0.0);
      }
    }
  }
}
