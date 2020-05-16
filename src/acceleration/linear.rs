use super::Acceleration;
use object::{Interact, Interaction, Object};
use ray::Ray;

pub struct Linear {
  list: Vec<Object>,
}

impl Interact for Linear {
  fn interact<'a>(&'a self, ray: &'a Ray) -> Option<Interaction> {
    self.list.iter().flat_map(|v| v.interact(&ray)).min()
  }
}

impl Acceleration for Linear {}

impl Linear {
  pub fn new(objects: Vec<Object>) -> Self {
    Linear { list: objects }
  }
}
