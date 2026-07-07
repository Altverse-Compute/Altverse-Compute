use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Normal {
  entity: Entity,
}

impl Normal {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 0;
    Self { entity }
  }
}

impl EntityLogic for Normal {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
  }

  fn get_changes(&self) -> Vec<EntityField> {
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
