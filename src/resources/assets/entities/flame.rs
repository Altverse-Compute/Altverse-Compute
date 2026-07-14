use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Flame {
  entity: Entity,
  timer: f32,
}

impl Flame {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 18;
    Self { entity, timer: 0.0 }
  }
}

impl EntityLogic for Flame {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f32;
    if self.timer >= 32.0 * ((self.entity.radius * 2.0) / self.entity.speed) {
      let mut trail = FlameTrail::new(
        EntityProps {
          id: 1,
          type_id: 19,
          radius: self.entity.radius,
          speed: 0.0,
          boundary: self.entity.boundary,
        },
        AdditionalEntityProps {
          count: 0,
          num: 0,
          inverse: false,
        },
      );
      trail.entity.pos = self.entity.pos.clone();
      trail.entity.changed_pos();

      trail.owner_speed = self.entity.speed;
      self.timer = 0.0;
      props.event_bus.add_entity(EntityWrapper::FlameTrail(trail));
    }
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
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

#[derive(Clone)]
pub struct FlameTrail {
  pub entity: Entity,
  timer: f32,
  pub owner_speed: f32,
}

impl FlameTrail {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
      timer: 0.0,
      owner_speed: 0.0,
    }
  }
}

impl EntityLogic for FlameTrail {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    let duration = 5000.0 / self.owner_speed;
    self.timer += props.delta as f32;
    self.entity.alpha = (1.0 - self.timer / duration).clamp(0.0, 1.0);
    self.entity.changed_alpha();
    if self.timer >= 5000.0 / self.owner_speed {
      self.entity.to_remove = true;
    }
    if self.timer >= 3500.0 {
      self.entity.harmless = true;
      self.entity.changed_harmless();
    }
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
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
