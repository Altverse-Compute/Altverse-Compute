use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::ids::MAGNETIC_SOUL_ID;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::player::Player;
use crate::resources::utils::vector::Vector;
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps, distance};

const MAX_DIST: f32 = 120f32;
const CHASE_SPEED: f32 = 2.5f32;

#[derive(Clone)]
pub struct MagneticSoul {
  entity: Entity,
  pub start_position: Vector,
  pub timeout: f32,
  pub caster_id: u64,
  was_interacted: bool,
}

impl MagneticSoul {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.vel = Vector::new(Some(0f32), Some(0f32));
    entity.type_id = MAGNETIC_SOUL_ID;
    Self {
      start_position: entity.pos.clone(),
      entity,
      timeout: 3000f32,
      caster_id: 0,
      was_interacted: false,
    }
  }

  fn move_back_to_origin(&mut self) {
    let dist = distance(
      self.entity.pos.x - self.start_position.x,
      self.entity.pos.y - self.start_position.y,
    );

    if dist < 1f32 {
      self.entity.vel.x = 0f32;
      self.entity.vel.y = 0f32;
    }

    let angle =
      (self.start_position.y - self.entity.pos.y).atan2(self.start_position.x - self.entity.pos.x);

    self.entity.vel.x = angle.cos() * 2f32;
    self.entity.vel.y = angle.sin() * 2f32;
  }
}

impl EntityLogic for MagneticSoul {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.timeout -= props.delta;
    if self.timeout <= 0f32 {
      println!("Timeout is dead");
      self.entity.to_remove = true;
      return;
    }

    let mut target: Option<&&mut Player> = None;
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
      let dx = target.pos.x - self.entity.pos.x;
      let dy = target.pos.y - self.entity.pos.y;

      let angle = dy.atan2(dx);
      let dir_x = angle.cos();
      let dir_y = angle.sin();

      let next_x = self.entity.pos.x + dir_x * CHASE_SPEED * props.time_fix;
      let next_y = self.entity.pos.y + dir_y * CHASE_SPEED * props.time_fix;

      let dist_from_origin = distance(
        next_x - self.start_position.x,
        next_y - self.start_position.y,
      );

      if dist_from_origin > MAX_DIST {
        self.move_back_to_origin();
        return;
      }

      self.entity.vel.x = dir_x * CHASE_SPEED;
      self.entity.vel.y = dir_y * CHASE_SPEED;
    } else {
      self.move_back_to_origin();
    }

    if self.was_interacted {
      for player in props.players.iter_mut() {
        if player.id == self.caster_id {
          player.res();
          println!("Player was saved");
        }
        println!("Player id {}, caster id {}", player.id, self.caster_id);
      }
      self.entity.to_remove = true;
    }

    for player in props.players.iter_mut() {
      if player.id == self.caster_id && !player.downed {
        self.entity.to_remove = true;
      }
    }

    self.entity.update(props);
  }

  fn interact(&mut self, hero: &mut HeroWrapper) {
    let player = hero.player_mut();
    if !self.entity.harmless
      && player.pos.x > -player.radius
      && player.pos.x - player.radius < self.entity.boundary.w
    {
      if !player.immortal && !player.downed {
        if distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        ) <= self.entity.radius + player.radius
        {
          if hero.player().id != self.caster_id {
            self.was_interacted = true;
          }
        }
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
