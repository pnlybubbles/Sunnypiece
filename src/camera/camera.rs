use ray::Ray;
use sample::Sample;
use sample::pdf;
use object::Transform;

pub trait Camera: Transform {
  type PDF: pdf::Measure;

  fn sample(&self, u: f32, v: f32) -> Sample<Ray, Self::PDF>;
}
