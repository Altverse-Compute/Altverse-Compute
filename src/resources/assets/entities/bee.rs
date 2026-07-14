use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::player::Player;
use crate::resources::{distance, AdditionalEntityProps, EntityProps, EntityUpdateProps};
use std::f64::consts::PI;

const MAX_DIST: f32 = 5.625 * 32.0;
const ANGLE_INCREMENT: f32 = 0.04;

#[derive(Clone)]
pub struct Bee {
  entity: Entity,
}

impl Bee {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 15;
    Self { entity }
  }
}

impl EntityLogic for Bee {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    let mut target: Option<&&Player> = None;
    let mut last_distance = MAX_DIST;
    for player in props.players.iter() {
      if player.pos.x > -player.radius
        && player.pos.x - player.radius < self.entity.boundary.w
        && !player.downed
      {
        let dist = distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        );
        if dist <= MAX_DIST && dist < last_distance {
          last_distance = dist;
          target = Some(player);
        }
      }
    }

    if let Some(target) = target {
      let d_x = target.pos.x - self.entity.pos.x;
      let d_y = target.pos.y - self.entity.pos.y;
      let target_angle = d_y.atan2(d_x);

      self.entity.vel_to_angle();

      let mut angle_diff = target_angle - self.entity.angle;

      if angle_diff > PI as f32 {
        angle_diff -= 2.0 * PI as f32;
      }
      if angle_diff < -PI as f32 {
        angle_diff += 2.0 * PI as f32;
      }

      let max_turn = ANGLE_INCREMENT * (props.delta as f32 / 16.67);
      if angle_diff.abs() < max_turn {
        self.entity.angle = target_angle;
      } else {
        self.entity.angle += angle_diff.sin() * max_turn;
      }

      self.entity.angle_to_vel();
    }

    self.entity.update(props);
    self.entity.collide();
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
