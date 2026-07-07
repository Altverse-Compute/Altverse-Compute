use crate::bus::{EventBus, NetworkBus};
use crate::fbs::Package;
use crate::resources::assets::effect::PlayerEffectWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::utils::join::JoinProps;
use crate::resources::world::World;
use crate::resources::{PlayerUpdateProps, UpdateProps};
use napi::{Error, Status};
use std::collections::HashMap;

pub struct PlayersManager {
  pub players: HashMap<u64, HeroWrapper>,
  pub players_diff: Vec<u64>,
  pub players_to_remove: Vec<u64>,
  pub effects: HashMap<u64, HashMap<u64, PlayerEffectWrapper>>,
  pub effects_to_remove: Vec<u64>,
}

impl PlayersManager {
  pub fn new() -> Self {
    Self {
      players: HashMap::new(),
      players_diff: Vec::new(),
      players_to_remove: Vec::new(),
      effects_to_remove: Vec::new(),
      effects: HashMap::new(),
    }
  }

  pub fn join(
    &mut self,
    player_props: &JoinProps,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
  ) -> Result<(), Error> {
    let hero = HeroWrapper::new("maven", player_props.clone())?;
    let player = hero.player().clone();
    let player_id = hero.player().id;
    let world_name = hero.player().world.clone();

    self.players.insert(player_id, hero);
    if let Some(_) = self.players.get(&player_id) {
      if let Some(world) = worlds.get_mut(&world_name) {
        world.join(&player);

        network_bus.add_global_package(Package::NewPlayer(player.id));
        network_bus.add_direct_package(
          player.id,
          Package::AreaInit(world.pack_area(player.area as usize)),
        );

        let players = self.pack_players();

        network_bus.add_direct_package(player_id, Package::Players(players));

        network_bus.add_direct_package(player_id, Package::Myself(player.id));

        return Ok(());
      }
    }

    Err(Error::new(
      Status::InvalidArg,
      format!("World not found: {}", world_name),
    ))
  }

  pub fn leave(
    &mut self,
    player_id: u64,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
  ) {
    if let Some(hero) = self.players.get(&player_id) {
      let player = hero.player();
      if let Some(world) = worlds.get_mut(&player.world) {
        world.leave(&player);
      }
      self.players.remove(&player_id);
    }
    network_bus.add_global_package(Package::ClosePlayer(player_id));
  }

  pub fn snapshot_end(&mut self, network_bus: &mut NetworkBus) {
    self.players_diff.clear();
    for (index, hero) in self.players.iter() {
      if !hero.get_changes().is_empty() {
        self.players_diff.push(*index);
      }
    }
    if !self.players_diff.is_empty() {
      network_bus.add_global_package(Package::UpdatePlayers(self.players_diff.clone()));
    }
    self.players_diff.clear();
  }

  pub fn update_behavior(
    &mut self,
    update_props: &UpdateProps,
    worlds: &mut HashMap<String, World>,
    network_bus: &mut NetworkBus,
    event_bus: &mut EventBus,
  ) {
    let players_clone = &self.players.clone();

    for (_, effects) in self.effects.iter_mut() {
      effects.retain(|_, effect_w| {
        if effect_w.effect().to_remove {
          effect_w.disable(self.players.get_mut(&effect_w.effect().target_id).unwrap());
          false
        } else {
          true
        }
      });
    }

    for (id, hero) in self.players.iter_mut() {
      let player = hero.player();
      if let Some(worlds) = worlds.get_mut(&player.world) {
        if let Some(area) = worlds.areas.get_mut(player.area as usize) {
          let mut update_player_props = PlayerUpdateProps {
            time_fix: update_props.time_fix,
            delta: update_props.delta,
            players: area.get_players_vec(&players_clone),
            event_bus,
          };
          hero.update(&mut update_player_props);
          let boundary = area.as_boundary_player();
          hero.collide(boundary);
          if let Some(client) = network_bus.direct_clients.get_mut(id) {
            hero.input(&mut client.input);
          }
        }
      }
    }

    event_bus.process_players_events(self);
  }

  pub fn add_player_effect(&mut self, effect: &mut PlayerEffectWrapper) {
    if !self.has_player_effect(effect.effect_id(), effect.effect().target_id) {
      if let Some(player) = self.players.get_mut(&effect.effect().target_id) {
        if let Some(effects) = self.effects.get_mut(&(effect.effect().target_id)) {
          effect.enable(player);
          effects.insert(effect.effect_id(), effect.clone());
        } else {
          let mut effects: HashMap<u64, PlayerEffectWrapper> = HashMap::new();
          effect.enable(player);
          effects.insert(effect.effect_id(), effect.clone());
          self.effects.insert(effect.effect().target_id, effects);
        }
      }
    }
  }

  pub fn has_player_effect(&mut self, self_id: u64, target_id: u64) -> bool {
    if let Some(effects) = self.effects.get(&target_id) {
      if let Some(_) = effects.get(&self_id) {
        return true;
      }
    }
    false
  }

  pub fn check_players_to_remove(&mut self) -> Vec<u64> {
    self.players_to_remove.clear();

    for (id, hero) in self.players.iter_mut() {
      if hero.player().to_delete {
        self.players_to_remove.push(*id);
      }
    }

    self.players_to_remove.clone()
  }

  // pub fn update_interact(
  //   &mut self,
  //   update_props: &UpdateProps,
  //   worlds: &mut HashMap<String, World>,
  // ) {
  //   for (_, player) in self.players.iter_mut() {
  //     if let Some(worlds) = worlds.get_mut(&player.world) {
  //       if let Some(area) = worlds.areas.get_mut(&player.area as usize) {
  //         for (_, ent) in area.entities.iter_mut() {}
  //       }
  //     }
  //   }
  // }

  pub fn get_player(&self, id: u64) -> Option<&HeroWrapper> {
    self.players.get(&id)
  }

  pub(crate) fn pack_players(&self) -> Vec<u64> {
    let mut result = Vec::new();

    for (id, _) in self.players.iter() {
      result.push(*id);
    }

    result
  }

  pub fn clean_changes(&mut self) {
    for (_, player) in self.players.iter_mut() {
      player.clear_changes();
    }
  }
}
