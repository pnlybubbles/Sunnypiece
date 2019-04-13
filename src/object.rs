use math::Matrix4;

pub trait Transform {
  fn transform(&self) -> &Matrix4;
}
