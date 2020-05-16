use math::*;

pub trait Transform {
  fn transform(&self) -> &Matrix4;
}
