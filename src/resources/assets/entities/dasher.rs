use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::ids::DASHER_ID;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps, distance};

#[derive(Clone)]
pub struct Dasher {
  entity: Entity,
  default_speed: f32,
}

impl Dasher {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = DASHER_ID;
    Self {
      entity,
      default_speed: props.speed,
    }
  }
}

impl EntityLogic for Dasher {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();
    if self.entity.speed != self.default_speed {
      self.entity.vel_to_angle();
      self.entity.speed = self.default_speed;
      self.entity.angle_to_vel();
    }
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
    let player = player.player_mut();
    if distance(
      player.pos.x - self.entity.pos.x,
      player.pos.y - self.entity.pos.y,
    ) <= 32f32 * 5f32
      && !player.downed
    {
      self.entity.vel_to_angle();
      self.entity.speed = self.default_speed * 5f32;
      self.entity.angle_to_vel();
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
