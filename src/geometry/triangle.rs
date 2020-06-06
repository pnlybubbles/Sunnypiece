use geometry::{Geometry, Intersection};
use math::*;
use ray::Ray;
use sample::{pdf, Sample};

pub struct Triangle {
  p0: Vector3,
  p1: Vector3,
  p2: Vector3,
  normal: Vector3,
  area: f32,
}

impl Triangle {
  pub fn new(p0: Vector3, p1: Vector3, p2: Vector3) -> Self {
    Triangle {
      p0: p0,
      p1: p1,
      p2: p2,
      normal: (p1 - p0).cross(p2 - p0).normalize(),
      area: (p1 - p0).cross(p2 - p0).norm() * 0.5,
    }
  }
}

impl Geometry for Triangle {
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    // Möller–Trumbore intersection algorithm
    let e1 = self.p1 - self.p0;
    let e2 = self.p2 - self.p0;
    let pv = ray.direction.cross(e2);
    let det = e1.dot(pv); // クラメルの分母
    if det.abs() < EPS {
      return None;
    }
    let invdet = 1.0 / det;
    let tv = ray.origin - self.p0;
    let u = tv.dot(pv) * invdet;
    if u < 0.0 || u > 1.0 {
      return None;
    }
    let qv = tv.cross(e1);
    let v = ray.direction.dot(qv) * invdet;
    if v < 0.0 || u + v > 1.0 {
      return None;
    }
    let t = e2.dot(qv) * invdet;
    if t < EPS {
      return None;
    }
    let p = ray.origin + ray.direction * t;
    Some(Intersection {
      distance: t,
      normal: self.normal,
      position: p,
    })
  }

  fn area(&self) -> f32 {
    self.area
  }

  fn sample(&self) -> Sample<Vector3, pdf::Area> {
    let u = rand::random::<f32>();
    let v = rand::random::<f32>();
    let min = u.min(v);
    let max = u.max(v);
    Sample {
      value: self.p0 * min + self.p1 * (1.0 - max) + self.p2 * (max - min),
      pdf: pdf::Area(1.0 / self.area),
    }
  }

  fn pdf(&self) -> pdf::Area {
    return pdf::Area(1.0 / self.area);
  }
}
