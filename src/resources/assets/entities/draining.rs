use crate::bus::PlayerEvent;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps, distance};

#[derive(Clone)]
pub struct Draining {
  entity: Entity,
  players_in_aura: Vec<u64>,
}

impl Draining {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 26;
    entity.state = 1;
    entity.state_metadata = 150.0;
    entity.aura = 150.0;
    Self {
      entity,
      players_in_aura: Vec::new(),
    }
  }
}

impl EntityLogic for Draining {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    for player_id in self.players_in_aura.iter() {
      props.event_bus.players_events.push(PlayerEvent::AddEffect {
        player_id: *player_id,
        effect_id: 1,
        caster_id: self.entity.id,
      })
    }
    self.players_in_aura.clear();
  }

  fn interact<'a>(&mut self, hero: &mut HeroWrapper) {
    let player = hero.player_mut();

    if distance(
      player.pos.x - self.entity.pos.x,
      player.pos.y - self.entity.pos.y,
    ) <= self.entity.aura + player.radius
    {
      self.players_in_aura.push(player.id);
    }

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
          player.knock();
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
