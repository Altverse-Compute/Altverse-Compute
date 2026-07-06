use crate::config::RawArea;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::player::Player;
use crate::resources::{random, AdditionalEntityProps, Boundary, EntityProps};
use std::collections::HashMap;

pub struct Area {
  pub entities: HashMap<u64, EntityWrapper>,
  pub players_id: Vec<i64>,
  pub raw_area: RawArea,
  pub next_id: u64,
}

impl Area {
  pub fn new(raw_area: RawArea) -> Self {
    Self {
      entities: HashMap::new(),
      players_id: Vec::new(),
      raw_area,
      next_id: 0,
    }
  }

  pub fn join(&mut self, id: i64) {
    if self.players_id.len() == 0 {
      self.init();
    }
    self.players_id.push(id);
  }

  pub fn leave(&mut self, id: i64) {
    self.players_id.retain(|&item| item != id);
    if self.players_id.len() == 0 {
      self.entities.clear();
    }
  }

  pub fn get_packed_entities(&self) -> HashMap<u64, &EntityWrapper> {
    let mut packed_entities: HashMap<u64, &EntityWrapper> = HashMap::new();

    for (id, entity) in self.entities.iter() {
      packed_entities.insert(*id, entity);
    }

    packed_entities
  }

  pub fn get_packed_entities_vec(&self) -> Vec<&EntityWrapper> {
    let mut packed_entities: Vec<&EntityWrapper> = Vec::new();

    for (_, entity) in self.entities.iter() {
      packed_entities.push(entity);
    }

    packed_entities
  }

  fn init(&mut self) {
    self.next_id = 0;
    if self.raw_area.enemies.len() != 0 {
      for entity in &self.raw_area.enemies {
        for num in 0..entity.count {
          let props = EntityProps {
            id: self.next_id,
            type_id: 0,
            radius: entity.radius,
            speed: entity.speed,
            boundary: Boundary {
              x: 0.0,
              y: 0.0,
              w: self.raw_area.w,
              h: self.raw_area.h,
            },
          };
          let type_name = entity
            .types
            .get(random(0.0, entity.types.len() as f64 - 1.0).round() as usize)
            .unwrap();
          let additional = AdditionalEntityProps {
            count: entity.count as u64,
            num: num as u64,
            inverse: false,
          };

          if let Ok(entity) = EntityWrapper::new(type_name.as_str(), &mut props.clone(), additional)
          {
            self.entities.insert(self.next_id, entity);
            self.next_id += 1;
          }
        }
      }
    }
  }

  pub fn add_entity(&mut self, entity: EntityWrapper) -> u64 {
    if self.next_id > 100_000 {
      self.next_id = 0;
    }
    self.next_id += 1;
    let mut entity_clone = entity.clone();
    entity_clone.entity_mut().id = self.next_id;
    self.entities.insert(self.next_id, entity_clone);
    self.next_id
  }

  pub fn get_players_vec<'a>(&self, players: &'a HashMap<i64, HeroWrapper>) -> Vec<&'a Player> {
    let mut arr = Vec::new();

    for id in &self.players_id {
      if let Some(player) = players.get(id) {
        arr.push(player.player());
      }
    }

    arr
  }

  pub fn as_boundary(&self) -> Boundary {
    Boundary {
      x: 0.0,
      y: 0.0,
      w: self.raw_area.w,
      h: self.raw_area.h,
    }
  }

  pub fn as_boundary_player(&self) -> Boundary {
    Boundary {
      x: -10.0 * 32.0,
      y: 0.0,
      w: self.raw_area.w + 20.0 * 32.0,
      h: self.raw_area.h,
    }
  }
}
