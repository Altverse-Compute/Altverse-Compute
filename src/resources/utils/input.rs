use crate::napi;

#[derive(Clone, Debug)]
#[napi]
pub struct Input {
  pub left: bool,
  pub right: bool,
  pub up: bool,
  pub down: bool,
  pub shift: bool,
  pub mouse_enable: bool,
  pub mouse_pos_x: f64,
  pub mouse_pos_y: f64,
  pub first_ability: bool,
  pub second_ability: bool,
}

#[napi]
impl Input {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      left: false,
      right: false,
      up: false,
      down: false,
      shift: false,
      mouse_enable: false,
      mouse_pos_x: 0.0,
      mouse_pos_y: 0.0,
      first_ability: false,
      second_ability: false,
    }
  }
  #[napi]
  pub fn set_left(&mut self, val: bool) {
    self.left = val;
  }

  #[napi]
  pub fn set_right(&mut self, val: bool) {
    self.right = val;
  }

  #[napi]
  pub fn set_up(&mut self, val: bool) {
    self.up = val;
  }

  #[napi]
  pub fn set_down(&mut self, val: bool) {
    self.down = val;
  }

  #[napi]
  pub fn set_shift(&mut self, val: bool) {
    self.shift = val;
  }

  #[napi]
  pub fn set_mouse_enable(&mut self, val: bool) {
    self.mouse_enable = val;
  }

  #[napi]
  pub fn set_mouse_pos_x(&mut self, val: f64) {
    self.mouse_pos_x = val;
  }

  #[napi]
  pub fn set_mouse_pos_y(&mut self, val: f64) {
    self.mouse_pos_y = val;
  }

  #[napi]
  pub fn set_first_ability(&mut self, val: bool) {
    self.first_ability = val;
  }

  #[napi]
  pub fn set_second_ability(&mut self, val: bool) {
    self.second_ability = val;
  }
}
