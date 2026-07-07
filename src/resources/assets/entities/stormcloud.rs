use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{distance, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct StormCloud {
  entity: Entity,
  time_fix: f32,
  timer: f32,
}

impl StormCloud {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 27;
    entity.alpha = 0.8;
    Self {
      entity,
      time_fix: 0.0,
      timer: 0.0,
    }
  }
}

impl EntityLogic for StormCloud {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
    self.time_fix = props.time_fix;
    self.timer = (self.timer % 2000.0) + props.delta;
    self.entity.alpha = ((self.timer / 1000.0).sin()).abs();
    self.entity.changed_alpha();
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
    let player = player.player_mut();
    if player.pos.x > player.radius && player.pos.x - player.radius < self.entity.boundary.w {
      if distance(
        self.entity.pos.x - player.pos.x,
        self.entity.pos.y - player.pos.y,
      ) <= self.entity.radius + player.radius
      {
        let dx = player.pos.x - self.entity.pos.x;
        let dy = player.pos.y - self.entity.pos.y;
        let dist = distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        );
        let attract_amplitude = 2 ^ -(dist / 120.0) as i32;
        let move_dist = (3 * attract_amplitude) as f32;
        let angle = dy.atan2(dx);
        player.pos.x += move_dist * angle.cos() * self.time_fix;
        player.pos.y += move_dist * angle.sin() * self.time_fix;
        player.changed_pos();
      }
    }
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
