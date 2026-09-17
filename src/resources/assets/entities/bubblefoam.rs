use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::ids::{BUBBLE_FOAM_ID, FADE_ID};
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps, distance};

#[derive(Clone)]
pub struct BubbleFoam {
  entity: Entity,
  was_to_remove: bool,
}

const BUBBLE_FOAM_CHANCE_TO_REMOVE: i32 = 60;

impl BubbleFoam {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.interactable_with_entities = true;
    entity.type_id = BUBBLE_FOAM_ID;
    entity.alpha = 0.3f32;
    Self {
      entity,
      was_to_remove: false,
    }
  }

  fn to_remove_chance(&mut self) {
    let integer = rand::random_range(0..100);
    if integer > BUBBLE_FOAM_CHANCE_TO_REMOVE {
      self.was_to_remove = true;
    }
  }

  fn collide(&mut self) {
    if self.entity.pos.x - self.entity.radius < self.entity.boundary.x {
      self.entity.pos.x = self.entity.boundary.x + self.entity.radius;
      self.entity.vel.x = self.entity.vel.x.abs();
      self.entity.changed_pos();
      self.to_remove_chance();
    }
    if self.entity.pos.x + self.entity.radius > self.entity.boundary.x + self.entity.boundary.w {
      self.entity.pos.x = self.entity.boundary.x + self.entity.boundary.w - self.entity.radius;
      self.entity.vel.x = -self.entity.vel.x.abs();
      self.entity.changed_pos();
      self.to_remove_chance();
    }
    if self.entity.pos.y - self.entity.radius < self.entity.boundary.y {
      self.entity.pos.y = self.entity.boundary.y + self.entity.radius;
      self.entity.vel.y = self.entity.vel.y.abs();
      self.entity.changed_pos();
      self.to_remove_chance();
    }
    if self.entity.pos.y + self.entity.radius > self.entity.boundary.y + self.entity.boundary.h {
      self.entity.pos.y = self.entity.boundary.y + self.entity.boundary.h - self.entity.radius;
      self.entity.vel.y = -(self.entity.vel.y.abs());
      self.entity.changed_pos();
      self.to_remove_chance();
    }
  }
}

impl EntityLogic for BubbleFoam {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.collide();
    if self.was_to_remove {
      self.entity.to_remove = true;
    }
  }

  fn interact(&mut self, _: &mut HeroWrapper) {}

  fn interact_with_entity(&mut self, entity: &mut EntityWrapper) {
    let entity = entity.entity_mut();
    if entity.immune {
      return;
    }
    if entity.type_id != BUBBLE_FOAM_ID
      && distance(
        entity.pos.x - self.entity.pos.x,
        entity.pos.y - self.entity.pos.y,
      ) <= self.entity.radius + entity.radius
    {
      entity.pos.x = self.entity.pos.x;
      entity.pos.y = self.entity.pos.y;
      entity.changed_pos();
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
