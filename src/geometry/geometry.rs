use ray::Ray;
use super::intersection::Intersection;

pub trait Geometry {
  fn intersect(&self, &Ray) -> Option<Intersection>;
}
