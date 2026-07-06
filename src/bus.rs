use crate::managers::player::PlayersManager;
use crate::proto::package::Kind;
use crate::proto::{Package, Packages};
use crate::resources::assets::effect::PlayerEffectWrapper;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::utils::input::Input;
use crate::resources::utils::vector::Vector;
use std::collections::HashMap;

pub struct Client {
  pub packages: Vec<Package>,
  pub input: Input,
}

pub struct NetworkBus {
  pub direct_clients: HashMap<i64, Client>,
  pub area_clients: HashMap<(String, u64), Packages>,
}

impl NetworkBus {
  pub fn new() -> Self {
    Self {
      direct_clients: HashMap::new(),
      area_clients: HashMap::new(),
    }
  }

  pub fn add_client(&mut self, player_id: i64) {
    self.direct_clients.insert(
      player_id,
      Client {
        input: Input::new(),
        packages: Packages { items: Vec::new() },
      },
    );
  }

  pub fn remove_client(&mut self, player_id: i64) {
    if let Some(_) = self.direct_clients.get(&player_id) {
      self.direct_clients.remove(&player_id);
    }
  }

  pub fn accept_input(&mut self, id: i64, input: &Input) {
    if let Some(client) = self.direct_clients.get_mut(&id) {
      client.input = input.clone();
    }
  }

  pub fn add_global_package(&mut self, package: Kind) {
    for client in self.direct_clients.values_mut() {
      client.packages.items.push(Package {
        kind: Some(package.clone()),
      });
    }
  }

  pub fn add_area_package(&mut self, name: String, area: u64, package: Kind) {
    if let Some(area) = self.area_clients.get_mut(&(name.clone(), area)) {
      area.items.push(Package {
        kind: Some(package),
      });
    } else {
      self
        .area_clients
        .insert((name, area), Packages { items: Vec::new() });
    }
  }

  pub fn add_direct_package(&mut self, id: i64, package: Kind) {
    if let Some(client) = self.direct_clients.get_mut(&id) {
      client.packages.items.push(Package {
        kind: Some(package),
      });
    }
  }

  pub fn clear_packages(&mut self) {
    for (_, client) in self.direct_clients.iter_mut() {
      client.packages.items.clear();
    }
  }
}

#[derive(Clone)]
pub enum PlayerEvent {
  ResPlayerAndMove {
    player_id: i64,
    pos: Vector,
  },
  AddEffect {
    player_id: i64,
    effect_id: u64,
    caster_id: u64,
  },
}

pub struct EventBus {
  pub entities_to_spawn: Vec<EntityWrapper>,
  pub players_events: Vec<PlayerEvent>,
}

impl EventBus {
  pub fn new() -> Self {
    Self {
      entities_to_spawn: Vec::new(),
      players_events: Vec::new(),
    }
  }

  pub fn add_entity(&mut self, entity: EntityWrapper) {
    self.entities_to_spawn.push(entity);
  }

  pub fn respawn_player_and_move(&mut self, player_id: i64, pos: Vector) {
    self
      .players_events
      .push(PlayerEvent::ResPlayerAndMove { player_id, pos });
  }

  pub fn process_players_events(&mut self, manager: &mut PlayersManager) {
    for event in self.players_events.iter() {
      match event {
        PlayerEvent::ResPlayerAndMove { player_id, pos } => {
          let players = &mut manager.players;
          if let Some(player) = players.get_mut(player_id) {
            player.res();
          }
        }
        PlayerEvent::AddEffect {
          effect_id,
          player_id,
          caster_id,
        } => {
          if let Some(hero) = manager.players.get(player_id) {
            // if !manager.has_player_effect(*effect_id, *player_id) {
            if let Ok(effect) = &mut PlayerEffectWrapper::new(*effect_id, hero, *caster_id) {
              manager.add_player_effect(effect);
            }
            // }
          }
        }
      }
    }
    self.players_events.clear();
  }
}
