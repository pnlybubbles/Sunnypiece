use math::Vector3;
use ray::Ray;

pub trait Radiance {
  fn radiance(&self, Ray) -> Vector3;
}
