use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Sizer {
  entity: Entity,
  min_radius: f32,
  max_radius: f32,
  growing: bool,
}

impl Sizer {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    let radius = entity.radius;
    entity.type_id = 24;
    Self {
      entity,
      min_radius: radius * 2.5,
      max_radius: radius / 2.5,
      growing: true,
    }
  }
}

impl EntityLogic for Sizer {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
    if self.growing {
      self.entity.radius += (props.time_fix * 0.08) * self.min_radius;
      self.entity.changed_radius();
      if self.entity.radius > self.max_radius {
        self.growing = false;
      }
    } else {
      self.entity.radius -= ((props.delta / 30.0) * 0.08) * self.min_radius;
      self.entity.changed_radius();
      if self.entity.radius < self.min_radius {
        self.growing = true;
      }
    }
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
