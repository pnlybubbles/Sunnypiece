use super::LightSampler;
use acceleration::Acceleration;
use geometry::Geometry;
use geometry::Intersection;
use material::Material;
use math::*;
use ray::Ray;
use sample::{pdf, Sample};
use std::cmp::Ordering;
use util::*;

pub trait Interact {
  fn interact<'a>(&'a self, ray: Ray) -> Option<Interaction>;
}

pub struct Interaction<'a> {
  material: &'a Box<dyn Material + Send + Sync>,
  geometry: &'a Box<dyn Geometry + Send + Sync>,
  pub intersection: Intersection,
  ray: Ray,
  pub orienting_normal: Vector3,
  is_backface: bool,
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
    let dot_sign = intersection.normal.dot(-ray.direction).signum();
    let orienting_normal = dot_sign * intersection.normal;
    debug_assert!(
      intersection.distance.abs() >= EPS,
      "{}",
      intersection.distance
    );
    Interaction {
      intersection: intersection,
      material: material,
      geometry: geometry,
      ray: ray,
      orienting_normal: orienting_normal,
      is_backface: dot_sign == -1.0,
    }
  }

  pub fn emittance(&self) -> Vector3 {
    if self.is_backface {
      Vector3::zero()
    } else {
      self.material.emittance()
    }
  }

  pub fn sample_material(&self) -> Sample<Vector3, pdf::SolidAngle> {
    let n = self.orienting_normal;
    let wi = -self.ray.direction;
    // BRDFに応じたサンプリング
    // NOTE: 方向ベクトルがサンプリングされる
    self.material.sample(wi, n, self.is_backface)
  }

  pub fn connect_direction<S>(&self, structure: &'a S, wo: Vector3) -> Option<Geom>
  where
    S: Acceleration,
  {
    let x = self.intersection.position;
    // 新しいレイ
    let ray = Ray {
      from: Some(self.geometry.id()),
      origin: x + self.orienting_normal * EPS,
      direction: wo,
    };
    debug_assert!(wo.is_finite(), "{}", wo);
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
      from: Some(self.geometry.id()),
      origin: x + self.orienting_normal * EPS,
      direction: path.normalize(),
    };
    debug_assert!(ray.direction.is_finite(), "{}", ray.direction);
    structure.interact(ray).and_then(|interaction| {
      // 可視チェック(2)
      if !interaction.intersection.distance.approx_eq(path.norm()) {
        return None;
      }
      // 可視チェック(3)
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
  pub wi: Vector3,
  pub wo: Vector3,
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
    let wo = path_i / next.intersection.distance;
    let wi = path_o / current.intersection.distance;
    if -wo.dot(n2) <= 0.0 {
      return None;
    }
    debug_assert!(wi.dot(n) > 0.0);
    Some(Geom {
      x: x,
      x_offset: x_offset,
      x1: x1,
      x2: x2,
      n: n,
      n2: n2,
      wi: wi,
      wo: wo,
      path_o: path_o,
      path_i: path_i,
      current: current,
      next: next,
    })
  }

  pub fn bsdf(&self) -> Vector3 {
    self
      .current
      .material
      .brdf(self.wi, self.wo, self.n, self.x, self.current.is_backface)
  }

  pub fn bsdf_pdf(&self) -> pdf::SolidAngle {
    self
      .current
      .material
      .pdf(self.wi, self.wo, self.n, self.current.is_backface)
  }

  pub fn g(&self) -> f32 {
    self.wo.dot(self.n) * (-self.wo).dot(self.n2) / (self.x2 - self.x_offset).sqr_norm()
  }

  pub fn light_pdf(&self, light_sampler: &LightSampler) -> Option<pdf::Area> {
    if self.next.material.emittance().sqr_norm() > 0.0 {
      light_sampler.pdf(self.next.geometry)
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
      (self.wo.dot(self.n) / p).is_finite(),
      "{}",
      self.wo.dot(self.n) / p
    );
    self.wo.dot(self.n) / p
  }
}

impl<'a> GeomWeight<pdf::Area> for Geom<'a> {
  fn weight(&self, pdf: pdf::Area) -> f32 {
    let pdf::Area(p) = pdf;
    debug_assert!(
      (self.wo.dot(self.n) * (-self.wo).dot(self.n2) / (self.x2 - self.x_offset).sqr_norm() / p)
        .is_finite(),
      "\nwo . n = {}\n-wo . n2 = {}\n|x2 - x| = {}\np = {}\n",
      self.wo.dot(self.n),
      (-self.wo).dot(self.n2),
      (self.x2 - self.x_offset).sqr_norm(),
      p
    );
    self.wo.dot(self.n) * (-self.wo).dot(self.n2) / (self.x2 - self.x_offset).sqr_norm() / p
  }
}
