use crate::resources::player::{Player, PlayerField};
use crate::resources::utils::input::Input;
use crate::resources::{Boundary, PlayerUpdateProps};

pub mod maven;

pub trait Hero {
  fn update(&mut self, props: &mut PlayerUpdateProps);
  fn input(&mut self, input: &mut Input);
  fn knock(&mut self);
  fn res(&mut self);
  fn collide(&mut self, boundary: Boundary);
  fn get_changes(&self) -> u32;
  fn clear_changes(&mut self);
  fn player(&self) -> &Player;
  fn player_mut(&mut self) -> &mut Player;
}
