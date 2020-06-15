use super::radiance::Radiance;
use acceleration::{Acceleration, AccelerationUtility};
use math::*;
use object::{GeomWeight, Interaction, LightSampler};
use ray::Ray;

pub struct OnlyLight<'a, S>
where
  S: Acceleration,
{
  structure: &'a S,
  light_sampler: LightSampler<'a>,
}

impl<'a, S> OnlyLight<'a, S>
where
  S: Acceleration + 'a,
{
  pub fn new(structure: &'a S) -> Self {
    OnlyLight {
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
          // 接続先から再帰的にパスを生成して散乱成分の寄与を蓄積する
          let li_scatter = self.radiance_recursive(&geom.next, depth + 1);
          let scatter_contrib = li_scatter * geom.bsdf() * geom.weight(material_sample.pdf);
          scatter_contrib
        }
      };

    // 明示的に光源の座標をサンプリング
    let light_sample = self
      .light_sampler
      .sample(point.intersection.position, point.intersection.normal);
    // 衝突点と光源を接続して光源サブパスを生成
    let light_oriented_contrib = light_sample
      .map(
        |sample| match point.connect_point(self.structure, sample.value) {
          None => Vector3::zero(),
          Some(geom) => {
            // 明示的な光源サブパスの重点的サンプリング
            let li = geom.next.emittance();
            let light_pdf = sample.pdf;
            li * geom.bsdf() * geom.weight(light_pdf)
          }
        },
      )
      .unwrap_or(Vector3::zero());

    le + material_oriented_contrib + light_oriented_contrib
  }
}

impl<'a, S> Radiance for OnlyLight<'a, S>
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
