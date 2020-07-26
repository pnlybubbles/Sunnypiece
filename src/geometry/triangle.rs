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
  n0: Vector3,
  n1: Vector3,
  n2: Vector3,
  normal: Vector3,
  area: f32,
  aabb: AABB,
  id: usize,
  bounding_sphere: (Vector3, f32),
}

impl Triangle {
  pub fn new(
    p0: Vector3,
    p1: Vector3,
    p2: Vector3,
    n0: Vector3,
    n1: Vector3,
    n2: Vector3,
    uuid: &mut UUID,
  ) -> Self {
    Triangle {
      p0,
      p1,
      p2,
      n0,
      n1,
      n2,
      normal: Self::normal(p0, p1, p2),
      area: (p1 - p0).cross(p2 - p0).norm() * 0.5,
      aabb: Self::aabb(p0, p1, p2),
      id: uuid.gen(),
      bounding_sphere: Self::bounding_sphere(p0, p1, p2),
    }
  }

  pub fn normal(p0: Vector3, p1: Vector3, p2: Vector3) -> Vector3 {
    (p1 - p0).cross(p2 - p0).normalize()
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

  fn bounding_sphere(p0: Vector3, p1: Vector3, p2: Vector3) -> (Vector3, f32) {
    let mut a_ = (p0 - p1).norm();
    let mut b_ = (p1 - p2).norm();
    let mut c_ = (p2 - p0).norm();

    // Re-orient triangle (make A longest side)
    let mut a = p2;
    let mut b = p0;
    let mut c = p1;

    if b_ < c_ {
      std::mem::swap(&mut b_, &mut c_);
      std::mem::swap(&mut b, &mut c);
    }
    if a_ < b_ {
      std::mem::swap(&mut a_, &mut b_);
      std::mem::swap(&mut a, &mut b);
    }

    // If obtuse, just use longest diameter, otherwise circumscribe
    if (b_ * b_) + (c_ * c_) <= (a_ * a_) {
      let position = (b + c) / 2.0;
      let radius = a_ / 2.0;
      (position, radius)
    } else {
      // http://en.wikipedia.org/wiki/Circumscribed_circle
      let cos_a = (b_ * b_ + c_ * c_ - a_ * a_) / (b_ * c_ * 2.0);
      let alpha = a - c;
      let beta = b - c;
      let position = (beta * alpha.dot(alpha) - alpha * beta.dot(beta)).cross(alpha.cross(beta))
        / (alpha.cross(beta).dot(alpha.cross(beta)) * 2.0)
        + c;
      let radius = a_ / ((1.0 - cos_a * cos_a).sqrt() * 2.0);
      (position, radius)
    }
  }
}

impl Geometry for Triangle {
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    if ray.from.map(|id| id == self.id).unwrap_or(false) {
      // Self-intersection
      return None;
    }
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
    // let g = x.barycentric_coordinate(self.p0, self.p1, self.p2);
    // (g.x * self.n0 + g.y * self.n1 + g.z * self.n2).normalize()
    self.normal
  }

  fn id(&self) -> usize {
    self.id
  }

  fn bounding_sphere(&self) -> (Vector3, f32) {
    self.bounding_sphere
  }
}
