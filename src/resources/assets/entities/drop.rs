use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::utils::vector::Vector;
use crate::resources::{random, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Drop {
  entity: Entity,
  time_at_some_surface: f32,
  speed_time: f32,
  start_time: f32,
  spawned: bool,
}

impl Drop {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.type_id = 7;
    entity.vel.x = 0.0;
    entity.vel.y = entity.speed;
    Self {
      entity,
      time_at_some_surface: 0.0,
      speed_time: 500.0,
      start_time: 0.0,
      spawned: true,
    }
  }

  fn re_spawn(&mut self) {
    self.entity.pos.x = random(
      self.entity.radius,
      self.entity.boundary.w - self.entity.radius,
    );
    self.entity.pos.y = self.entity.radius + 1.0;
    self.entity.vel = Vector::new(None, None);
    self.time_at_some_surface = random(1000.0, 2000.0);
    self.start_time = self.time_at_some_surface + 0.0;
    self.entity.harmless = true;
    self.entity.changed_pos();
    self.entity.changed_harmless();
  }

  fn collide(&mut self) {
    let entity = &mut self.entity;
    if entity.pos.x - entity.radius < entity.boundary.x {
      entity.pos.x = entity.boundary.x + entity.radius;
      entity.vel.x = entity.vel.x.abs();
      entity.changed_pos();
    }
    if entity.pos.x + entity.radius > entity.boundary.x + entity.boundary.w {
      entity.pos.x = entity.boundary.x + entity.boundary.w - entity.radius;
      entity.vel.x = -(entity.vel.x.abs());
      entity.changed_pos();
    }
    if entity.pos.y - entity.radius < entity.boundary.y {
      entity.pos.y = entity.boundary.y + entity.radius;
      entity.vel.y = entity.vel.y.abs();
      entity.changed_pos();
    }
    if entity.pos.y + entity.radius > entity.boundary.y + entity.boundary.h {
      entity.pos.y -= 1.0;
      entity.vel.y = 0.0;
      self.time_at_some_surface = -1000.0;
      self.start_time = 1000.0;
      self.spawned = false;
      entity.changed_pos();
    }
  }
}

impl EntityLogic for Drop {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.collide();

    if self.time_at_some_surface > 0.0 {
      self.time_at_some_surface -= props.delta as f32;
      self.entity.vel.y = 0.0;
      self.entity.alpha = 1.0 - self.time_at_some_surface / self.start_time;
      self.entity.harmless = true;
      if self.time_at_some_surface <= 0.0 {
        self.time_at_some_surface = 0.0;
        self.entity.harmless = false;
        self.entity.vel.y = self.entity.speed;
        self.speed_time = 500.0;
      }
      self.entity.changed_alpha();
      self.entity.changed_harmless();
    } else if self.time_at_some_surface < 0.0 {
      self.entity.vel.y = 0.0;
      self.entity.alpha = -self.time_at_some_surface / self.start_time;
      self.entity.changed_alpha();
      self.time_at_some_surface += props.delta as f32;
      if self.time_at_some_surface >= 0.0 {
        self.re_spawn();
      }
    } else {
      self.speed_time -= props.delta as f32;
      if self.spawned {
        self.entity.vel.y = self.entity.speed;
      } else {
        self.entity.vel.y = self.entity.speed;
        if self.speed_time > 0.0 {
          self.entity.vel.y *= 1.0 - self.speed_time / 500.0;
        }
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
