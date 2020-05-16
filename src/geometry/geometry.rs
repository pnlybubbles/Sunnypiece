use super::intersection::Intersection;
use ray::Ray;

pub trait Geometry {
  fn intersect(&self, &Ray) -> Option<Intersection>;
}
