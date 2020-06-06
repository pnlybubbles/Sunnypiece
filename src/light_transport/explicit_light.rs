use super::radiance::Radiance;
use acceleration::{Acceleration, AccelerationUtility};
use math::*;
use object::{GeomWeight, Interaction, LightSampler};
use ray::Ray;
use sample::mis::MIS;
use util::*;

pub struct ExplicitLight<'a, S>
where
  S: Acceleration,
{
  structure: &'a S,
  light_sampler: LightSampler<'a>,
}

impl<'a, S> ExplicitLight<'a, S>
where
  S: Acceleration + 'a,
{
  pub fn new(structure: &'a S) -> Self {
    ExplicitLight {
      structure: structure,
      light_sampler: structure.light_sampler(),
    }
  }

  fn radiance_recursive(&self, point: &Interaction, depth: usize) -> Vector3 {
    // 視線サブパスが直接光源に接続された場合のみ寄与を取る
    // 光源からの寄与は前のパスで取っているので含めない
    let le = if depth == 0 {
      point.emittance()
    } else {
      Vector3::zero()
    };
    // スタックオーバーフロー防止
    // TODO: ロシアンルーレット
    if depth > 5 {
      return le;
    }

    // マテリアルに基づいて方向ベクトルをサンプリング
    let material_sample = point.sample_material();
    // 衝突点から方向ベクトルを使ってパスを接続
    let material_oriented_contrib =
      match point.connect_direction(self.structure, material_sample.value) {
        None => Vector3::zero(),
        Some(geom) => {
          // マテリアルに比例した光源サブパスの重点的サンプリング
          let li = geom.next.emittance();
          let light_pdf = geom.light_pdf(&self.light_sampler);
          debug_assert!(light_pdf.map(|v| v.0).unwrap_or(0.0).is_finite());
          let bsdf_pdf = geom.bsdf_pdf();
          debug_assert!(bsdf_pdf.0.is_finite());
          let light_contrib = light_pdf
            .map(|pdf| {
              let mis_weight = bsdf_pdf.power_hulistic(pdf, 2);
              debug_assert!(mis_weight.is_finite());
              debug_assert!(geom.bsdf().is_finite());
              debug_assert!(geom.weight(bsdf_pdf).is_finite());
              li * geom.bsdf() * geom.weight(bsdf_pdf) * mis_weight
            })
            .unwrap_or(Vector3::zero());
          debug_assert!(light_contrib.is_finite());
          // 接続先から再帰的にパスを生成して散乱成分の寄与を蓄積する
          let li_scatter = self.radiance_recursive(&geom.next, depth + 1);
          let scatter_contrib = li_scatter * geom.bsdf() * geom.weight(material_sample.pdf);
          debug_assert!(scatter_contrib.is_finite());
          light_contrib + scatter_contrib
        }
      };

    // 明示的に光源の座標をサンプリング
    let light_sample = self.light_sampler.sample();
    // 衝突点と光源を接続して光源サブパスを生成
    let light_oriented_contrib = match point.connect_point(self.structure, light_sample.value) {
      None => Vector3::zero(),
      Some(geom) => {
        // 明示的な光源サブパスの重点的サンプリング
        let li = geom.next.emittance();
        let light_pdf = light_sample.pdf;
        debug_assert!(light_pdf.0.is_finite());
        let bsdf_pdf = geom.bsdf_pdf();
        debug_assert!(bsdf_pdf.0.is_finite());
        let mis_weight = light_pdf.power_hulistic(bsdf_pdf, 2);
        debug_assert!(mis_weight.is_finite());
        li * geom.bsdf() * geom.weight(light_pdf) * mis_weight
      }
    };

    le + material_oriented_contrib + light_oriented_contrib
  }
}

impl<'a, S> Radiance for ExplicitLight<'a, S>
where
  S: Acceleration,
{
  fn radiance(&self, ray: Ray) -> Vector3 {
    let maybe_interaction = self.structure.interact(ray);

    match maybe_interaction {
      None => Vector3::zero(),
      Some(interaction) => self.radiance_recursive(&interaction, 0),
    }
  }
}
