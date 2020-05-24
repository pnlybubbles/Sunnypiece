use super::geometry::Geometry;
use super::intersection::Intersection;
use math::*;
use ray::Ray;
use sample::{pdf, Sample};
use sampler::Sampler;

pub struct Sphere {
  position: Vector3,
  radius: f32,
  area: f32,
}

impl Sphere {
  pub fn new(position: Vector3, radius: f32) -> Self {
    Sphere {
      position: position,
      radius: radius,
      area: 4.0 * PI * radius.powi(2),
    }
  }
}

impl Geometry for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    let po = ray.origin - self.position;
    let b = ray.direction.dot(po);
    let c = po.sqr_norm() - self.radius * self.radius;
    // 判別式 Δ = b^2 - a*c
    let det = b * b - c;
    // 交差しない
    if det < 0.0 {
      return None;
    }
    let t1 = -b - det.sqrt();
    let t2 = -b + det.sqrt();
    // 出射方向と反対側で交差
    if t2 < EPS {
      return None;
    }
    // 近い方が正の場合はそれを採用
    // それ以外(球体内部からの交差の場合)は正の方を採用
    let distance = if t1 > EPS { t1 } else { t2 };
    // r = o + t * d
    let position = ray.origin + ray.direction * distance;
    // 法線は球体中心から外向き
    let normal = (position - self.position).normalize();
    Some(Intersection {
      position: position,
      normal: normal,
      distance: distance,
    })
  }

  fn area(&self) -> f32 {
    self.area
  }

  fn sample(&self) -> Sample<Vector3, pdf::Area> {
    Sample {
      value: self.position + self.radius * Sampler::sphere_uniform(),
      pdf: pdf::Area(1.0 / self.area),
    }
  }
}
