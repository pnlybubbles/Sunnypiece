use super::Acceleration;
use geometry::{Geometry, Intersection};
use object::Object;
use ray::Ray;

pub struct Linear {
  list: Vec<Object>,
}

impl Geometry for Linear {
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    self
      .list
      .iter()
      .flat_map(|v| v.geometry.intersect(&ray))
      .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
  }
}

impl Acceleration for Linear {}

impl Linear {
  pub fn new(objects: Vec<Object>) -> Self {
    Linear { list: objects }
  }
}
