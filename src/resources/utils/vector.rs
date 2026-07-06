use crate::resources::random;
use napi_derive::napi;

#[derive(Clone, Debug)]
#[napi]
pub struct Vector {
  pub x: f64,
  pub y: f64,
}

impl Vector {
  pub fn new(x: Option<f64>, y: Option<f64>) -> Self {
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

  pub fn from_mult(vec: Vector, mult: f64) -> Self {
    Self {
      x: vec.x * mult,
      y: vec.y * mult,
    }
  }

  pub fn rand(xs: f64, ys: f64, xe: f64, ye: f64) -> Self {
    Self {
      x: random(xs, xe),
      y: random(ys, ye),
    }
  }

  pub fn from_angle(angle: f64, multi: f64) -> Self {
    Self {
      x: angle.cos() * multi,
      y: angle.sin() * multi,
    }
  }
}
