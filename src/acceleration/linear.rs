use super::Acceleration;
use object::{Interact, Interaction, Object};
use ray::Ray;

pub struct Linear<'a> {
  list: Vec<Object<'a>>,
}

impl<'a> Interact for Linear<'a> {
  fn interact<'b>(&'b self, ray: Ray) -> Option<Interaction> {
    self.list.iter().flat_map(|v| v.interact(ray.clone())).min()
  }
}

impl<'a> Acceleration for Linear<'a> {
  fn objects(&self) -> &Vec<Object> {
    &self.list
  }
}

impl<'a> Linear<'a> {
  pub fn new(objects: Vec<Object<'a>>) -> Self {
    Linear { list: objects }
  }
}
