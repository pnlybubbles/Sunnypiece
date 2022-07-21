use super::radiance::Radiance;
use acceleration::Acceleration;
use math::*;
use ray::Ray;
use util::IsNormalized;

pub struct Id<S>
where
  S: Acceleration,
{
  structure: S,
}

impl<S> Id<S>
where
  S: Acceleration,
{
  pub fn new(structure: S) -> Self {
    Id { structure }
  }
}

impl<S> Radiance for Id<S>
where
  S: Acceleration,
{
  fn radiance(&self, ray: Ray) -> Vector3 {
    debug_assert!(
      ray.direction.is_normalized(),
      "ray direction should be normalized."
    );

    let maybe_interaction = self.structure.interact(ray);

    match maybe_interaction {
      None => Vector3::zero(),
      Some(interaction) => {
        let id = interaction.geometry_id();
        return hsl2rgb((id % 16) as f32 / 16.0, 1.0, 1.0);
      }
    }
  }
}

fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
  let mut t_ = t;
  if t_ < 0.0 {
    t_ += 1.0
  }
  if t_ > 1.0 {
    t_ -= 1.0
  }
  if t_ < 1.0 / 6.0 {
    return p + (q - p) * 6.0 * t_;
  }
  if t_ < 1.0 / 2.0 {
    return q;
  }
  if t_ < 2.0 / 3.0 {
    return p + (q - p) * (2.0 / 3.0 - t_) * 6.0;
  }
  return p;
}

fn hsl2rgb(h: f32, s: f32, l: f32) -> Vector3 {
  if s == 0.0 {
    // achromatic
    return Vector3::fill(l);
  } else {
    let q = if l < 0.5 {
      l * (1.0 + s)
    } else {
      l + s - l * s
    };
    let p = 2.0 * l - q;
    let r = hue2rgb(p, q, h + 1.0 / 3.0);
    let g = hue2rgb(p, q, h);
    let b = hue2rgb(p, q, h - 1.0 / 3.0);
    return Vector3::new(r, g, b);
  }
}
