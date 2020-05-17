use camera::Camera;
use math::*;
use object::Transform;
use ray::Ray;
use sample::*;

type Rad = f32;

pub struct IdealPinhole {
  aspect: f32,
  aperture: Vector3,
  aperture_to_film_distance: f32,
  matrix: Matrix4,
}

impl IdealPinhole {
  pub fn new(
    xfov: Rad,
    // width / height
    aspect: f32,
    matrix: Matrix4,
  ) -> IdealPinhole {
    // 開口部の位置
    let aperture = &matrix * Vector3::zero();
    // 視野角から開口部から撮像素子までの距離を計算
    // 撮像素子の大きさは1x(1/aspect)
    let aperture_to_film_distance = 0.5 / (xfov / 2.0).tan();
    IdealPinhole {
      aspect: aspect,
      aperture: aperture,
      aperture_to_film_distance: aperture_to_film_distance,
      matrix: matrix,
    }
  }
}

impl Transform for IdealPinhole {
  fn transform(&self) -> &Matrix4 {
    &self.matrix
  }
}

impl Camera for IdealPinhole {
  type PDF = pdf::SolidAngle;

  fn sample(&self, u: f32, v: f32) -> Sample<Ray, Self::PDF> {
    // サンプリング点の位置
    let point = Vector3::new(
      // センサー面はz正にあり、開口部は原点にある
      // uv座標系は像を正立させたときの左下を基準に定義される
      // センサーをウラ面から開口部方向(z負方向)に見たときに、右がx正、上がy正になる
      // 像は上下左右反転するので、正立像の左側がx正、下側がy正になる
      // uが0のとき、xが0.5
      0.5 - u,
      // vが0のとき、yが0.5
      (0.5 - v) / self.aspect,
      self.aperture_to_film_distance,
    );
    // レイの方向
    let direction = self.aperture - self.transform() * point;
    let ray = Ray {
      origin: self.aperture,
      direction: direction.normalize(),
    };
    Sample {
      value: ray,
      pdf: pdf::SolidAngle(distribution::DELTA_FUNCTION),
    }
  }
}
