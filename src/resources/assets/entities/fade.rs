use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Fade {
  entity: Entity,
  timer: f64,
}

const MAX_TIME: f64 = 7500.0;
const START_TIME: f64 = 7500.0 / 2.0;

impl Fade {
  pub fn new(props: EntityProps, additional: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    let mut timer = 0.0;
    if additional.num > additional.count / 2 {
      timer = START_TIME;
    }
    entity.type_id = 23;
    Self { entity, timer }
  }
}

impl EntityLogic for Fade {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;

    let period = MAX_TIME;

    let phase = (self.timer / period) * std::f64::consts::TAU;
    self.entity.alpha = (phase.cos() + 1.0) * 0.5;
    self.entity.harmless = self.entity.alpha < 0.5;
    self.entity.changed_alpha();
    self.entity.changed_harmless();
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
