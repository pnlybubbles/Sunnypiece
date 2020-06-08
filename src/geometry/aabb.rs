use math::*;
use ray::Ray;
use std::f32;
use util::*;

#[derive(Clone)]
pub struct AABB {
  pub min: Vector3,
  pub max: Vector3,
  pub center: Vector3,
}

impl AABB {
  pub fn side(&self) -> Vector3 {
    Vector3::new(
      (self.max.x - self.min.x).abs(),
      (self.max.y - self.min.y).abs(),
      (self.max.z - self.min.z).abs(),
    )
  }

  pub fn surface_area(&self) -> f32 {
    let side = self.side();
    2.0 * (side.x * side.y + side.y * side.z + side.z * side.x)
  }

  pub fn merge(list: &Vec<&AABB>) -> AABB {
    let min = Vector3::new(
      list.iter().map(|v| v.min.x).min_by(&unsafe_cmp).unwrap(),
      list.iter().map(|v| v.min.y).min_by(&unsafe_cmp).unwrap(),
      list.iter().map(|v| v.min.z).min_by(&unsafe_cmp).unwrap(),
    );
    let max = Vector3::new(
      list.iter().map(|v| v.max.x).max_by(&unsafe_cmp).unwrap(),
      list.iter().map(|v| v.max.y).max_by(&unsafe_cmp).unwrap(),
      list.iter().map(|v| v.max.z).max_by(&unsafe_cmp).unwrap(),
    );
    AABB {
      min: min,
      max: max,
      center: (min + max) / 2.0,
    }
  }

  pub fn merge_with(&self, v: &AABB) -> AABB {
    let min = Vector3::new(
      self.min.x.min(v.min.x),
      self.min.y.min(v.min.y),
      self.min.z.min(v.min.z),
    );
    let max = Vector3::new(
      self.max.x.max(v.max.x),
      self.max.y.max(v.max.y),
      self.max.z.max(v.max.z),
    );
    AABB {
      min: min,
      max: max,
      center: (min + max) / 2.0,
    }
  }

  pub fn empty() -> AABB {
    AABB {
      min: Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
      max: Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
      center: Vector3::zero(),
    }
  }

  #[inline]
  pub fn is_intersect(&self, ray: &Ray) -> bool {
    let mut min = -INF;
    let mut max = INF;
    for i in 0..3 {
      let inv_d = 1.0 / ray.direction[i];
      let t1 = (self.min[i] - ray.origin[i]) * inv_d;
      let t2 = (self.max[i] - ray.origin[i]) * inv_d;
      let (t_min, t_max) = if t1 > t2 { (t2, t1) } else { (t1, t2) };
      if min < t_min {
        min = t_min
      }
      if max > t_max {
        max = t_max
      }
      if min > max {
        return false;
      }
    }
    true
  }
}
