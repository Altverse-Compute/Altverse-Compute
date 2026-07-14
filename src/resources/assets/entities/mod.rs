use crate::resources::EntityUpdateProps;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};

pub mod bee;
pub mod cloud;
pub mod draining;
pub mod drop;
pub mod fade;
pub mod flame;
pub mod flamesniper;
pub mod homing;
pub mod homingsniper;
pub mod icicle;
pub mod immune;
pub mod leaf;
pub mod normal;
pub mod sizer;
pub mod slow;
pub mod sniper;
pub mod stormcloud;
pub mod wall;

pub trait EntityLogic {
  fn update(&mut self, props: &mut EntityUpdateProps);
  fn interact(&mut self, player: &mut HeroWrapper);
  fn get_changes(&self) -> u8;
  fn clear_changes(&mut self);
  fn entity(&self) -> &Entity;
  fn entity_mut(&mut self) -> &mut Entity;
}
