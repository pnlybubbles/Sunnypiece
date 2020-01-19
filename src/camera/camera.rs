use object::Transform;
use ray::Ray;
use sample::pdf;
use sample::Sample;

pub trait Camera: Transform {
  type PDF: pdf::Measure;

  fn sample(&self, u: f32, v: f32) -> Sample<Ray, Self::PDF>;
}
