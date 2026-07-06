use crate::resources::random;
use napi_derive::napi;

#[derive(Clone, Debug)]
#[napi]
pub struct Vector {
  pub x: f32,
  pub y: f32,
}

impl Vector {
  pub fn new(x: Option<f32>, y: Option<f32>) -> Self {
    Self {
      x: match x {
        Some(x) => x,
        _ => 0.0,
      },
      y: match y {
        Some(y) => y,
        _ => 0.0,
      },
    }
  }

  pub fn from_mult(vec: Vector, mult: f32) -> Self {
    Self {
      x: vec.x * mult,
      y: vec.y * mult,
    }
  }

  pub fn rand(xs: f32, ys: f32, xe: f32, ye: f32) -> Self {
    Self {
      x: random(xs as f64, xe as f64) as f32,
      y: random(ys as f64, ye as f64) as f32,
    }
  }

  pub fn from_angle(angle: f32, multi: f32) -> Self {
    Self {
      x: angle.cos() * multi,
      y: angle.sin() * multi,
    }
  }
}
