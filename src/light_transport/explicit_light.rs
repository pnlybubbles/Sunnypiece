use super::radiance::Radiance;
use acceleration::{Acceleration, AccelerationUtility};
use math::*;
use object::{GeomWeight, Interaction, LightSampler};
use ray::Ray;

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
    // 光源からの寄与は光源サンプリングで取っているので寄与に含めない
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
    let material_throughput = match point.connect_direction(self.structure, material_sample.value) {
      None => Vector3::zero(),
      Some(geom) => {
        // 接続先から再帰的にパスを生成する
        let li = self.radiance_recursive(&geom.next, depth + 1);
        li * geom.bsdf() * geom.weight(material_sample.pdf)
      }
    };

    // 明示的に光源をサンプリング
    let light_sample = self.light_sampler.sample();
    // 衝突点と光源を接続して光源サブパスを生成
    let light_throughput = match point.connect_point(self.structure, light_sample.value) {
      None => Vector3::zero(),
      Some(geom) => {
        // 光源に接続したら寄与を取って終端
        let li = geom.next.emittance();
        li * geom.bsdf() * geom.weight(light_sample.pdf)
      }
    };

    le + material_throughput + light_throughput
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
