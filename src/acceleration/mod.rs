mod linear;

pub use self::linear::*;
use object::Interact;

pub trait Acceleration: Interact {}
