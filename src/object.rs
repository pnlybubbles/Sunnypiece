use acceleration::Acceleration;
use camera::Camera;
use geometry::Geometry;
use math::Matrix4;

pub trait Transform {
  fn transform(&self) -> &Matrix4;
}

pub struct Object {
  pub geometry: Box<dyn Geometry>,
  matrix: Matrix4,
  // TODO: material
}

impl Object {
  pub fn new(geometry: Box<dyn Geometry>, matrix: Matrix4) -> Self {
    Object {
      geometry: geometry,
      matrix: matrix,
    }
  }
}

impl Transform for Object {
  fn transform(&self) -> &Matrix4 {
    &self.matrix
  }
}

pub struct Scene<T, S>
where
  T: Camera,
  S: Acceleration,
{
  pub camera: T,
  pub structure: S,
}
