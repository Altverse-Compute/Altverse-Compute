use crate::config::RawWorld;
use crate::resources::area::Area;
use crate::resources::player::Player;

pub struct World {
  raw_world: RawWorld,
  pub areas: Vec<Area>,
}

impl World {
  pub fn new(raw_world: RawWorld) -> Self {
    let mut areas = Vec::new();
    for a in &raw_world.areas {
      areas.push(Area::new(a.clone()));
    }
    Self { raw_world, areas }
  }

  pub fn join(&mut self, player: &Player) {
    if let Some(area) = self.areas.get_mut(player.area as usize) {
      area.join(player.id);
    }
  }

  pub fn leave(&mut self, player: &Player) {
    if let Some(area) = self.areas.get_mut(player.area as usize) {
      area.leave(player.id);
    }
  }
}
