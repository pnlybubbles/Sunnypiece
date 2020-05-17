use super::Acceleration;
use object::{Interact, Interaction, Object};
use ray::Ray;

pub struct Linear<'a> {
  list: Vec<Object<'a>>,
}

impl<'a> Interact for Linear<'a> {
  fn interact<'b>(&'b self, ray: &'b Ray) -> Option<Interaction> {
    self.list.iter().flat_map(|v| v.interact(&ray)).min()
  }
}

impl<'a> Acceleration for Linear<'a> {}

impl<'a> Linear<'a> {
  pub fn new(objects: Vec<Object<'a>>) -> Self {
    Linear { list: objects }
  }
}
