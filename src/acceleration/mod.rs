mod linear;

pub use self::linear::*;
use geometry::Geometry;

pub trait Acceleration: Geometry {}
