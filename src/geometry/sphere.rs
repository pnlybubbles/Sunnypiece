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
    let d = ray.direction;
    let f = ray.origin - self.position;
    let b = d.dot(f);
    let r2 = self.radius * self.radius;
    // 判別式 Δ = b^2 - a*c = (d.f)^2 - (f^2 - r^2)
    // f^2 - r^2 で半径よりもはるか遠い点からの衝突の場合に情報落ちが起こる
    // Hearn and Baker equation
    // 変形 Δ = r^2 - (f - d(d.f))^2
    let det = r2 - (f - d * b).sqr_norm();
    // 交差しない
    if det < 0.0 {
      return None;
    }
    // b ~ √Δ の場合に桁落ちが起こる
    // 桁落ちが起こらない方の解を使って導出する
    let c = f.sqr_norm() - r2;
    let q = -b - b.signum() * det.sqrt();
    let t1 = q;
    let t2 = c / q;
    let (tl, tg) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
    // 出射方向と反対側で交差
    if tg < EPS {
      return None;
    }
    // 近い方が正の場合はそれを採用
    // それ以外(球体内部からの交差の場合)は正の方を採用
    let distance = if tl > EPS { tl } else { tg };
    // r = o + t * d
    let position = ray.origin + ray.direction * distance;
    // 法線は球体中心から外向き
    let normal = (position - self.position).normalize();
    let position_refined = self.position + normal * self.radius;
    let distance_refined = (position_refined - ray.origin).norm();
    if distance_refined < EPS {
      return None;
    }
    Some(Intersection {
      position: position_refined,
      normal: normal,
      distance: distance_refined,
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

  fn pdf(&self) -> pdf::Area {
    pdf::Area(1.0 / self.area)
  }
}
