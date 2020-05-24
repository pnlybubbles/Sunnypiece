use acceleration::Acceleration;
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
    ray: Ray,
  ) -> Self {
    Interaction {
      intersection: intersection,
      material: material,
      ray: ray,
    }
  }

  pub fn factor(&self) -> &Intersection {
    &self.intersection
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
    self.material.sample(wo, n)
  }

  pub fn connect_direction<S>(&self, structure: &'a S, wi: Vector3) -> Option<Relation>
  where
    S: Acceleration,
  {
    let x = self.intersection.position;
    let n = self.intersection.normal;
    // 新しいレイ
    let ray = Ray {
      direction: wi,
      origin: x + n * EPS,
    };
    structure
      .interact(ray)
      .map(|interaction| Relation::new(self, interaction))
  }

  pub fn connect_point<S>(&self, structure: &'a S, x2: Vector3) -> Option<Relation>
  where
    S: Acceleration,
  {
    let x = self.intersection.position;
    let n = self.intersection.normal;
    let path = x2 - x;
    // 可視チェック(1)
    if path.dot(self.intersection.normal) < 0.0 {
      return None;
    }
    let ray = Ray {
      origin: x + n * EPS,
      direction: path.normalize(),
    };
    structure.interact(ray).and_then(|interaction| {
      // 可視チェック(2)
      if interaction.intersection.distance.approx_eq(path.norm()) {
        Some(Relation::new(self, interaction))
      } else {
        None
      }
    })
  }
}

pub struct Relation<'a> {
  x: Vector3,
  x1: Vector3,
  x2: Vector3,
  n: Vector3,
  n2: Vector3,
  wo: Vector3,
  wi: Vector3,
  pub current: &'a Interaction<'a>,
  pub next: Interaction<'a>,
}

impl<'a> Relation<'a> {
  fn new(current: &'a Interaction, next: Interaction<'a>) -> Self {
    debug_assert!(current.intersection.position.approx_eq(next.ray.origin));
    Relation {
      x: current.intersection.position,
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
}

pub trait RelationWeight<PDF>
where
  PDF: pdf::Measure,
{
  fn weight(&self, pdf: PDF) -> f32;
}

impl<'a> RelationWeight<pdf::SolidAngle> for Relation<'a> {
  fn weight(&self, pdf: pdf::SolidAngle) -> f32 {
    let pdf::SolidAngle(p) = pdf;
    self.wi.dot(self.n) / p
  }
}

impl<'a> RelationWeight<pdf::Area> for Relation<'a> {
  fn weight(&self, pdf: pdf::Area) -> f32 {
    let pdf::Area(p) = pdf;
    self.wi.dot(self.n) * (-self.wi).dot(self.n2) / (self.x2 - self.x).sqr_norm() / p
  }
}
