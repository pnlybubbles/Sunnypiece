use super::Film;
use math::*;
use scarlet::color::RGBColor;
use scarlet::colormap::{ColorMap, ListedColorMap};
use util::*;

pub struct Linear;

pub struct Gamma {
  gamma: f32,
}

impl Default for Gamma {
  fn default() -> Self {
    Gamma { gamma: 2.2 }
  }
}

pub struct Debug {
  pub colormap: ListedColorMap,
  pub clamp: (f64, f64),
}

impl Default for Debug {
  fn default() -> Self {
    Debug {
      colormap: ListedColorMap::viridis(),
      clamp: (0.0, 1.0),
    }
  }
}

pub trait Tonemap {
  type Input;
  type Output;

  fn mapper(&self, film: &Film<Self::Input>) -> Box<dyn Fn(&Self::Input) -> Self::Output>;
}

impl Tonemap for Linear {
  type Input = Vector3;
  type Output = [u8; 3];

  fn mapper(&self, _film: &Film<Self::Input>) -> Box<dyn Fn(&Self::Input) -> Self::Output> {
    Box::new(|input| {
      let correct = input.map(|v| v.min(1.0).max(0.0) * 255.0);
      [correct.x as u8, correct.y as u8, correct.z as u8]
    })
  }
}

impl Tonemap for Gamma {
  type Input = Vector3;
  type Output = [u8; 3];

  fn mapper(&self, _film: &Film<Self::Input>) -> Box<dyn Fn(&Self::Input) -> Self::Output> {
    let gamma = self.gamma;
    Box::new(move |input| {
      let correct = input.map(|v| v.min(1.0).max(0.0).powf(1.0 / gamma) * 255.0);
      [correct.x as u8, correct.y as u8, correct.z as u8]
    })
  }
}

impl Tonemap for Debug {
  type Input = Vector3;
  type Output = [u8; 3];

  fn mapper(&self, film: &Film<Self::Input>) -> Box<dyn Fn(&Self::Input) -> Self::Output> {
    let max = film
      .data
      .iter()
      .max_by(|a, b| unsafe_cmp(&a.x, &b.x))
      .unwrap()
      .x;
    let min = film
      .data
      .iter()
      .min_by(|a, b| unsafe_cmp(&a.x, &b.x))
      .unwrap()
      .x;
    println!("max: {} min: {}", max, min);
    let clamp = self.clamp;
    let colormap = self.colormap.clone();
    Box::new(move |input| {
      let v = ((input.x - min) / (max - min)) as f64;
      let correct: RGBColor = colormap.transform_single((v - clamp.0) / (clamp.1 - clamp.0));
      [correct.int_r(), correct.int_g(), correct.int_b()]
    })
  }
}
