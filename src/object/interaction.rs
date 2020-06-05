use super::LightSampler;
use acceleration::Acceleration;
use geometry::Geometry;
use geometry::Intersection;
use material::Material;
use math::*;
use ray::Ray;
use sample::{pdf, Sample};
use std::cmp::Ordering;

pub trait Interact {
  fn interact<'a>(&'a self, ray: Ray) -> Option<Interaction>;
}

pub struct Interaction<'a> {
  material: &'a Box<dyn Material + Send + Sync>,
  geometry: &'a Box<dyn Geometry + Send + Sync>,
  intersection: Intersection,
  ray: Ray,
}

impl<'a> Eq for Interaction<'a> {}

impl<'a> PartialEq for Interaction<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.intersection.distance == other.intersection.distance
  }
}

impl<'a> PartialOrd for Interaction<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self
      .intersection
      .distance
      .partial_cmp(&other.intersection.distance)
  }
}

impl<'a> Ord for Interaction<'a> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(&other).unwrap()
  }
}

impl<'a> Interaction<'a> {
  pub fn new(
    intersection: Intersection,
    material: &'a Box<dyn Material + Send + Sync>,
    geometry: &'a Box<dyn Geometry + Send + Sync>,
    ray: Ray,
  ) -> Self {
    Interaction {
      intersection: intersection,
      material: material,
      geometry: geometry,
      ray: ray,
    }
  }

  pub fn normal(&self) -> Vector3 {
    self.intersection.normal
  }

  pub fn is_backface(&self) -> bool {
    self.intersection.normal.dot(-self.ray.direction) < 0.0
  }

  pub fn emittance(&self) -> Vector3 {
    self.material.emittance()
  }

  pub fn sample_material(&self) -> Sample<Vector3, pdf::SolidAngle> {
    let n = self.intersection.normal;
    let wo = -self.ray.direction;
    // BRDFに応じたサンプリング
    // NOTE: 方向ベクトルがサンプリングされる
    self.material.sample(wo, n)
  }

  pub fn connect_direction<S>(&self, structure: &'a S, wi: Vector3) -> Option<Geom>
  where
    S: Acceleration,
  {
    let x = self.intersection.position;
    // 新しいレイ
    let ray = Ray {
      direction: wi,
      origin: x,
    };
    structure
      .interact(ray)
      .map(|interaction| Geom::new(self, interaction))
  }

  pub fn connect_point<S>(&self, structure: &'a S, x2: Vector3) -> Option<Geom>
  where
    S: Acceleration,
  {
    let x = self.intersection.position;
    let path = x2 - x;
    // 可視チェック(1)
    if path.dot(self.intersection.normal) < 0.0 {
      return None;
    }
    let ray = Ray {
      origin: x,
      direction: path.normalize(),
    };
    structure.interact(ray).and_then(|interaction| {
      // 可視チェック(2)
      if interaction.intersection.distance.approx_eq(path.norm()) {
        Some(Geom::new(self, interaction))
      } else {
        None
      }
    })
  }
}

pub struct Geom<'a> {
  x: Vector3,
  x_: Vector3,
  x1: Vector3,
  x2: Vector3,
  n: Vector3,
  n2: Vector3,
  wo: Vector3,
  wi: Vector3,
  pub current: &'a Interaction<'a>,
  pub next: Interaction<'a>,
}

impl<'a> Geom<'a> {
  fn new(current: &'a Interaction, next: Interaction<'a>) -> Self {
    debug_assert!(current.intersection.position.approx_eq(next.ray.origin));
    Geom {
      x: current.intersection.position,
      x_: next.ray.origin,
      x1: current.ray.origin,
      x2: next.intersection.position,
      n: current.intersection.normal,
      n2: next.intersection.normal,
      wo: -current.ray.direction,
      wi: next.ray.direction,
      current: current,
      next: next,
    }
  }

  pub fn bsdf(&self) -> Vector3 {
    self.current.material.brdf(self.wo, self.wi, self.n, self.x)
  }

  pub fn bsdf_pdf(&self) -> pdf::Area {
    self
      .current
      .material
      .pdf(self.wi, self.n)
      .area_measure(self.x_, self.x2, self.n2)
  }

  pub fn g(&self) -> f32 {
    self.wi.dot(self.n) * (-self.wi).dot(self.n2) / (self.x2 - self.x_).sqr_norm()
  }

  pub fn light_pdf(&self, light_sampler: &LightSampler) -> Option<pdf::Area> {
    if self.next.material.emittance().norm() > 0.0 {
      Some(light_sampler.pdf(self.next.geometry))
    } else {
      None
    }
  }
}

pub trait GeomWeight<PDF>
where
  PDF: pdf::Measure,
{
  fn weight(&self, pdf: PDF) -> f32;
}

impl<'a> GeomWeight<pdf::SolidAngle> for Geom<'a> {
  fn weight(&self, pdf: pdf::SolidAngle) -> f32 {
    let pdf::SolidAngle(p) = pdf;
    debug_assert!(
      (self.wi.dot(self.n) / p).is_finite(),
      "{}",
      self.wi.dot(self.n) / p
    );
    self.wi.dot(self.n) / p
  }
}

impl<'a> GeomWeight<pdf::Area> for Geom<'a> {
  fn weight(&self, pdf: pdf::Area) -> f32 {
    let pdf::Area(p) = pdf;
    self.wi.dot(self.n) * (-self.wi).dot(self.n2) / (self.x2 - self.x_).sqr_norm() / p
  }
}
