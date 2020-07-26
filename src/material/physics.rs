use math::*;

pub trait BoundaryResponse
where
  Self: Sized,
{
  fn reflect(&self, normal: Self) -> Self;
  fn refract(&self, normal: Self, from_per_to_ior: f32) -> Option<Self>;
}

impl BoundaryResponse for Vector3 {
  fn reflect(&self, normal: Vector3) -> Vector3 {
    -*self + normal * ((*self).dot(normal) * 2.0)
  }

  fn refract(&self, normal: Vector3, from_per_to_ior: f32) -> Option<Vector3> {
    let dn = self.dot(normal);
    let cos2theta = 1.0 - from_per_to_ior.powi(2) * (1.0 - dn.powi(2));
    if cos2theta > 0.0 {
      Some(-*self * from_per_to_ior - normal * (from_per_to_ior * -dn + cos2theta.sqrt()))
    } else {
      None
    }
  }
}

pub struct Fresnel;

impl Fresnel {
  pub fn schlick(f0: Vector3, wo: Vector3, n: Vector3) -> Vector3 {
    f0 + (Vector3::fill(1.0) - f0) * (1.0 - wo.dot(n)).powi(5)
  }
}
