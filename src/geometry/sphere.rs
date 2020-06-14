use super::geometry::Geometry;
use super::intersection::Intersection;
use super::AABB;
use super::UUID;
use math::*;
use ray::Ray;
use sample::{pdf, Sample};
use sampler::Sampler;

pub struct Sphere {
  position: Vector3,
  radius: f32,
  area: f32,
  aabb: AABB,
  id: usize,
}

impl Sphere {
  pub fn new(position: Vector3, radius: f32, uuid: &mut UUID) -> Self {
    Sphere {
      position: position,
      radius: radius,
      area: 4.0 * PI * radius.powi(2),
      aabb: Self::aabb(position, radius),
      id: uuid.gen(),
    }
  }

  fn aabb(position: Vector3, radius: f32) -> AABB {
    let r = Vector3::fill(radius);
    AABB {
      min: position - r,
      max: position + r,
      center: position,
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
    let c = f.sqr_norm() - r2;
    if c > 0.0 && b > 0.0 {
      // cが正のときはレイ始点が球の外
      // bが正ときは級の中心がレイ始点より後ろ
      return None;
    }
    // b ~ √Δ の場合に桁落ちが起こる
    // 解の公式で桁落ちが起こらない方の解を計算して、もう一方をcから導出する
    let q = -b - b.signum() * det.sqrt();
    let t1 = q;
    let t2 = c / q;
    // 球体の外側からの交差の場合は小さい方を採用
    // 球体の内側からの交差の場合は大きい方を採用
    let distance = if c > 0.0 { t1.min(t2) } else { t1.max(t2) };
    // r = o + t * d
    let position = ray.origin + ray.direction * distance;
    // 法線は球体中心から外向き
    let normal = (position - self.position).normalize();
    let position_refined = self.position + normal * self.radius;
    // 近すぎる衝突点は棄却 (self-intersection)
    if distance < EPS {
      return None;
    }
    Some(Intersection {
      position: position_refined,
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

  fn pdf(&self) -> pdf::Area {
    pdf::Area(1.0 / self.area)
  }

  fn aabb(&self) -> &AABB {
    &self.aabb
  }

  fn normal(&self, x: Vector3) -> Vector3 {
    (x - self.position).normalize()
  }

  fn id(&self) -> usize {
    self.id
  }
}
