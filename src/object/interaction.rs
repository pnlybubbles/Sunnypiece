use geometry::Intersection;
use material::Material;
use math::*;
use ray::Ray;
use sample::pdf;
use std::cmp::Ordering;

pub trait Interact {
  fn interact<'a>(&'a self, ray: &'a Ray) -> Option<Interaction>;
}

pub struct Interaction<'a> {
  material: &'a Box<dyn Material>,
  intersection: Intersection,
  ray: &'a Ray,
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
  pub fn new(intersection: Intersection, material: &'a Box<dyn Material>, ray: &'a Ray) -> Self {
    Interaction {
      intersection: intersection,
      material: material,
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

  pub fn material_throughput(&self) -> (Ray, Vector3) {
    let position = self.intersection.position;
    let out_vec = -self.ray.direction;
    let normal_vec = self.normal();
    // BRDFに応じたサンプリング
    let sample = self.material.sample(out_vec, normal_vec);
    let in_vec = sample.value;
    let pdf::SolidAngle(pdf) = sample.pdf;
    // BRDF
    let brdf = self.material.brdf(out_vec, in_vec, normal_vec, position);
    // コサイン項
    let cos = in_vec.dot(normal_vec);
    // レンダリング方程式に従って放射輝度の計算をする
    let throughput = brdf * cos / pdf;
    // 新しいレイ
    let ray = Ray {
      direction: in_vec,
      origin: position,
    };
    (ray, throughput)
  }
}
