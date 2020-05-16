use super::Interact;
use super::Interaction;
use super::Transform;
use geometry::Geometry;
use material::Material;
use math::*;
use ray::Ray;

pub struct Object {
  pub geometry: Box<dyn Geometry>,
  matrix: Matrix4,
  material: Box<dyn Material>,
}

impl Object {
  pub fn new(geometry: Box<dyn Geometry>, matrix: Matrix4, material: Box<dyn Material>) -> Self {
    Object {
      geometry: geometry,
      matrix: matrix,
      material: material,
    }
  }
}

impl Transform for Object {
  fn transform(&self) -> &Matrix4 {
    &self.matrix
  }
}

impl Interact for Object {
  fn interact<'a>(&'a self, ray: &'a Ray) -> Option<Interaction> {
    self
      .geometry
      .intersect(ray)
      .map(|intersection| Interaction::new(intersection, &self.material, &ray))
  }
}
