use crate::bus::PlayerEvent;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps, distance, random};

#[derive(Clone)]
pub struct Leaf {
  entity: Entity,
  time_spawn: f32,
  remove_time: f32,
  remove: bool,
  start_radius: f32,
  players: Vec<u64>,
}

impl Leaf {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 8;
    entity.vel.x = 0.0;
    entity.vel.y = 0.0;
    entity.harmless = true;
    let start_radius = entity.radius;
    Self {
      entity,
      time_spawn: 1000.0,
      remove_time: 500.0,
      remove: false,
      start_radius,
      players: Vec::new(),
    }
  }

  fn respawn(&mut self) {
    self.entity.pos.x = random(
      self.entity.radius + self.entity.boundary.x,
      self.entity.boundary.w,
    );
    self.entity.pos.y = random(
      self.entity.radius + self.entity.boundary.y,
      self.entity.boundary.h,
    );
    self.time_spawn = 1000.0;
    self.remove_time = 500.0;
    self.remove = false;
  }
}

impl EntityLogic for Leaf {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    if self.time_spawn > 0.0 {
      self.time_spawn -= props.delta;
      self.entity.radius = self.start_radius * 2.0 * 0.5f32.max(self.time_spawn / 1000.0);
      self.entity.alpha = 1.0 - self.time_spawn / 1000.0;
      self.entity.changed_alpha();
      self.entity.changed_radius();
    } else if self.time_spawn <= 0.0 && self.entity.harmless {
      self.entity.harmless = false;
      self.entity.radius = self.start_radius;
      self.entity.changed_radius();
      self.entity.changed_harmless();
      self.time_spawn = 0.0;
    }
    if self.remove {
      self.remove_time -= props.delta;
      self.entity.harmless = true;
      self.entity.alpha = self.remove_time / 500.0;
      self.entity.radius = self.start_radius * 2.0 * 0.5_f32.max(1.0 - self.remove_time / 500.0);
      self.entity.changed_alpha();
      self.entity.changed_radius();
      self.entity.changed_harmless();
      if self.remove_time < 0.0 {
        self.respawn();
      }
    }
    for id in self.players.iter() {
      props.event_bus.players_events.push(PlayerEvent::AddEffect {
        player_id: *id,
        effect_id: 2,
        caster_id: self.entity.id,
      })
    }
    self.players.clear();
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
          self.players.push(hero.player().id);
          self.remove = true;
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
