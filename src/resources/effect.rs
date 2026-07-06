use crate::resources::assets::hero::HeroWrapper;
use crate::resources::player::Player;

#[derive(Clone, Debug)]
pub struct PlayerEffect {
  pub to_remove: bool,
  pub id: u64,
  pub target_id: u64,
  pub caster_id: u64,
}

impl PlayerEffect {
  pub fn new(id: u64, target_id: u64, caster_id: u64) -> Self {
    Self {
      to_remove: false,
      id,
      target_id,
      caster_id,
    }
  }
  pub fn enable(&mut self, _: &mut Player) {}
  pub fn disable(&self, _: &mut HeroWrapper) {}
  pub fn update(&mut self) {}
}
