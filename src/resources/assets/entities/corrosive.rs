use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::ids::CORROSIVE_ID;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps, distance};

#[derive(Clone)]
pub struct Corrosive {
  entity: Entity,
}

impl Corrosive {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = CORROSIVE_ID;
    Self { entity }
  }
}

impl EntityLogic for Corrosive {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    let player = player.player_mut();
    if !self.entity.harmless
      && player.pos.x > -player.radius
      && player.pos.x - player.radius < self.entity.boundary.w
    {
      if distance(
        player.pos.x - self.entity.pos.x,
        player.pos.y - self.entity.pos.y,
      ) <= self.entity.radius + player.radius
      {
        player.knock();
      }
    }
  }

  fn get_changes(&self) -> u8 {
    self.entity.get_changes()
  }

  fn clear_changes(&mut self) {
    self.entity.clear_changes();
  }

  fn entity(&self) -> &Entity {
    &self.entity
  }

  fn entity_mut(&mut self) -> &mut Entity {
    &mut self.entity
  }
}
