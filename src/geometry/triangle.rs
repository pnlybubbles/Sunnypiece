use super::AABB;
use super::UUID;
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
  aabb: AABB,
  id: usize,
}

impl Triangle {
  pub fn new(p0: Vector3, p1: Vector3, p2: Vector3, uuid: &mut UUID) -> Self {
    Triangle {
      p0: p0,
      p1: p1,
      p2: p2,
      normal: (p1 - p0).cross(p2 - p0).normalize(),
      area: (p1 - p0).cross(p2 - p0).norm() * 0.5,
      aabb: Self::aabb(p0, p1, p2),
      id: uuid.gen(),
    }
  }

  fn aabb(p0: Vector3, p1: Vector3, p2: Vector3) -> AABB {
    let min = Vector3::new(
      p0.x.min(p1.x).min(p2.x),
      p0.y.min(p1.y).min(p2.y),
      p0.z.min(p1.z).min(p2.z),
    );
    let max = Vector3::new(
      p0.x.max(p1.x).max(p2.x),
      p0.y.max(p1.y).max(p2.y),
      p0.z.max(p1.z).max(p2.z),
    );
    AABB {
      min: min,
      max: max,
      center: (max + min) / 2.0,
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
    debug_assert!(det.is_finite(), "{}", det);
    let invdet = 1.0 / det;
    debug_assert!(invdet.is_finite(), "{}", invdet);
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
    debug_assert!(t.is_finite(), "{}", t);
    if t < EPS {
      return None;
    }
    // Derive position from barycentric coordinates
    let p = self.p0 + e1 * u + e2 * v;
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

  fn aabb(&self) -> &AABB {
    &self.aabb
  }

  fn normal(&self, _x: Vector3) -> Vector3 {
    self.normal
  }

  fn id(&self) -> usize {
    self.id
  }
}
