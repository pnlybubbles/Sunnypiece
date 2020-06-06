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
  pub orienting_normal: Vector3,
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
    let orienting_normal = intersection.normal.dot(-ray.direction).signum() * intersection.normal;
    Interaction {
      intersection: intersection,
      material: material,
      geometry: geometry,
      ray: ray,
      orienting_normal: orienting_normal,
    }
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
      origin: x + self.orienting_normal * EPS,
      direction: wi,
    };
    structure.interact(ray).and_then(|interaction| {
      // 可視チェック(1)
      Geom::new(self, interaction)
    })
  }

  pub fn connect_point<S>(&self, structure: &'a S, x2: Vector3) -> Option<Geom>
  where
    S: Acceleration,
  {
    let x = self.intersection.position;
    let path = x2 - x;
    // 可視チェック(1)
    if path.dot(self.orienting_normal) < 0.0 {
      return None;
    }
    let ray = Ray {
      origin: x + self.orienting_normal * EPS,
      direction: path.normalize(),
    };
    structure.interact(ray).and_then(|interaction| {
      // 可視チェック(2)
      if !interaction.intersection.distance.approx_eq(path.norm()) {
        return None;
      }
      Geom::new(self, interaction)
    })
  }
}

pub struct Geom<'a> {
  pub x: Vector3,
  pub x_offset: Vector3,
  pub x1: Vector3,
  pub x2: Vector3,
  pub n: Vector3,
  pub n2: Vector3,
  pub wo: Vector3,
  pub wi: Vector3,
  pub path_o: Vector3,
  pub path_i: Vector3,
  pub current: &'a Interaction<'a>,
  pub next: Interaction<'a>,
}

impl<'a> Geom<'a> {
  fn new(current: &'a Interaction, next: Interaction<'a>) -> Option<Self> {
    debug_assert!(current.intersection.position.approx_eq(next.ray.origin));
    let x = current.intersection.position;
    let x_offset = next.ray.origin;
    let x1 = current.ray.origin;
    let x2 = next.intersection.position;
    let n = current.orienting_normal;
    let n2 = next.orienting_normal;
    let path_o = x1 - x;
    let path_i = x2 - x_offset;
    let wi = path_i / next.intersection.distance;
    let wo = path_o / current.intersection.distance;
    if wi.dot(n) < 0.0 {
      return None;
    }
    if -wi.dot(n2) < 0.0 {
      return None;
    }
    debug_assert!(wo.dot(n) > 0.0);
    Some(Geom {
      x: x,
      x_offset: x_offset,
      x1: x1,
      x2: x2,
      n: n,
      n2: n2,
      wo: wo,
      wi: wi,
      path_o: path_o,
      path_i: path_i,
      current: current,
      next: next,
    })
  }

  pub fn bsdf(&self) -> Vector3 {
    self.current.material.brdf(self.wo, self.wi, self.n, self.x)
  }

  pub fn bsdf_pdf(&self) -> pdf::Area {
    self
      .current
      .material
      .pdf(self.wi, self.n)
      .area_measure(self.x_offset, self.x2, self.n2)
  }

  pub fn g(&self) -> f32 {
    self.wi.dot(self.n) * (-self.wi).dot(self.n2) / (self.x2 - self.x_offset).sqr_norm()
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
    debug_assert!(
      (self.wi.dot(self.n) * (-self.wi).dot(self.n2) / (self.x2 - self.x_offset).sqr_norm() / p)
        .is_finite(),
      "\nwi . n = {}\n-wi . n2 = {}\n|x2 - x| = {}\np = {}\n",
      self.wi.dot(self.n),
      (-self.wi).dot(self.n2),
      (self.x2 - self.x_offset).sqr_norm(),
      p
    );
    self.wi.dot(self.n) * (-self.wi).dot(self.n2) / (self.x2 - self.x_offset).sqr_norm() / p
  }
}
