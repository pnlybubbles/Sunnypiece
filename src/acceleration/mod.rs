mod bvh;
mod linear;

pub use self::bvh::*;
pub use self::linear::*;
use object::Interact;
use object::{LightSampler, Object};

pub trait Acceleration: Interact {
  fn objects(&self) -> &Vec<Object>;
}

pub trait AccelerationUtility: Acceleration {
  fn light_sampler(&self) -> LightSampler;
}

impl<T> AccelerationUtility for T
where
  T: Acceleration,
{
  fn light_sampler(&self) -> LightSampler {
    LightSampler::new(self.objects())
  }
}
