use super::Interact;
use super::Interaction;
use super::Transform;
use geometry::Geometry;
use material::Material;
use math::*;
use ray::Ray;

pub struct Object<'a> {
  pub geometry: Box<dyn Geometry + Send + Sync>,
  matrix: Matrix4,
  material: &'a Box<dyn Material + Send + Sync>,
}

impl<'a> Object<'a> {
  pub fn new(
    geometry: Box<dyn Geometry + Send + Sync>,
    matrix: Matrix4,
    material: &'a Box<dyn Material + Send + Sync>,
  ) -> Self {
    Object {
      geometry: geometry,
      matrix: matrix,
      material: material,
    }
  }
}

impl<'a> Transform for Object<'a> {
  fn transform(&self) -> &Matrix4 {
    &self.matrix
  }
}

impl<'a> Interact for Object<'a> {
  fn interact<'b>(&'b self, ray: &'b Ray) -> Option<Interaction> {
    self
      .geometry
      .intersect(ray)
      .map(|intersection| Interaction::new(intersection, &self.material, &ray))
  }
}
