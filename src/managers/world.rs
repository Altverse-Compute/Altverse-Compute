use crate::bus::{EventBus, NetworkBus};
use crate::config::Config;
use crate::fbs::Package;
use crate::managers::player::PlayersManager;
use crate::props::EngineProps;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::player::Player;
use crate::resources::world::World;
use crate::resources::{distance, EffectUpdateProps, EntityUpdateProps, UpdateProps};
use std::collections::HashMap;

pub struct WorldsManager {
  pub worlds: HashMap<String, World>,
  pub new_entities: Vec<u64>,
  pub old_entities: Vec<u64>,
  pub entities_diff: Vec<u64>,
  pub spawned_entities: Vec<u64>,
  pub entities_to_remove: Vec<u32>,
}

pub enum Change {
  NextArea,
  PrevArea,
  NextWorld,
  PrevWorld,
}

impl WorldsManager {
  pub fn new(props: &EngineProps) -> Self {
    Self {
      worlds: props.load_worlds().unwrap(),
      new_entities: Vec::new(),
      old_entities: Vec::new(),
      entities_diff: Vec::new(),
      spawned_entities: Vec::new(),
      entities_to_remove: Vec::new(),
    }
  }

  pub fn update(
    &mut self,
    props: &UpdateProps,
    players_manager: &mut PlayersManager,
    network_bus: &mut NetworkBus,
    event_bus: &mut EventBus,
  ) {
    let players_clone = &mut players_manager.players.clone();
    let players = &mut players_manager.players;
    for (name, world) in self.worlds.iter_mut() {
      for (index, area) in world.areas.iter_mut().enumerate() {
        self.old_entities = area.get_packed_entities();
        event_bus.entities_to_spawn.clear();
        let boundary = area.as_boundary();

        let mut entity_update = EntityUpdateProps {
          delta: props.delta,
          time_fix: props.time_fix,
          players: area.get_players_vec(players_clone),
          event_bus,
        };

        for (_, effects) in players_manager.effects.iter_mut() {
          for (_, effect) in effects.iter_mut() {
            let target_id = effect.effect().target_id.clone();
            let caster_id = effect.effect().caster_id.clone();

            if let Some(target) = players.get_mut(&target_id) {
              if let Some(caster) = area.entities.get_mut(&caster_id) {
                effect.update(&mut EffectUpdateProps {
                  delta: props.delta,
                  time_fix: props.time_fix,
                  caster,
                  target,
                  boundary,
                });
              }
            }
          }
        }

        area.entities.retain(|id, entity| {
          if entity.entity().to_remove {
            self.entities_to_remove.push(*id as u32);
            false
          } else {
            true
          }
        });

        for (_, entity) in area.entities.iter_mut() {
          entity.update(&mut entity_update);
        }

        for (_, entity) in area.entities.iter_mut() {
          for id in &area.players_id {
            if let Some(player) = players.get_mut(id) {
              entity.interact(player);
            }
          }
        }

        for first_id in &area.players_id {
          for second_id in &area.players_id {
            if first_id == second_id {
              continue;
            }

            let can_rescue = {
              let first_player = players.get(&first_id).unwrap().player();
              let second_player = players.get(&second_id).unwrap().player();
              second_player.downed
                && distance(
                  second_player.pos.x - first_player.pos.x,
                  second_player.pos.y - first_player.pos.y,
                ) <= first_player.radius + second_player.radius
            };

            if can_rescue {
              if let Some(second_player) = players.get_mut(&second_id) {
                second_player.res();
              }
            }
          }
        }

        for entity in event_bus.entities_to_spawn.iter() {
          let id = area.add_entity(entity.clone());
          self.spawned_entities.insert(id as usize, entity.pack());
        }

        self.new_entities = area.get_packed_entities();

        for (id, entity) in self.new_entities.iter() {
          if let Some(old_entity) = self.old_entities.get(&id) {
            let (diff, changed) = old_entity.diff(&entity);
            if changed {
              self.entities_diff.insert(*id, diff);
            }
          }
        }

        if !self.spawned_entities.is_empty() {
          let package = Package::NewEntities(self.spawned_entities.clone());
          network_bus.add_area_package(name.clone(), index as u64, package.clone());
          self.spawned_entities.clear();
        }

        if !self.entities_to_remove.is_empty() {
          let package = Package::CloseEntities(self.entities_to_remove.clone());
          network_bus.add_area_package(name.clone(), index as u64, package.clone());
          self.entities_to_remove.clear();
        }

        if !self.entities_diff.is_empty() {
          let package = Package::UpdateEntities(self.entities_diff.clone());
          network_bus.add_area_package(name.clone(), index as u64, package.clone());
          self.entities_diff.clear();
        }
      }
    }
  }

  pub fn prepare_warps(&mut self, players: &HashMap<u64, HeroWrapper>) -> HashMap<u64, Change> {
    let mut changes: HashMap<u64, Change> = HashMap::new();

    for (id, hero) in players.iter() {
      let player = hero.player();
      if let Some(world) = self.worlds.get(&player.world) {
        if let Some(area) = world.areas.get(player.area as usize) {
          if player.pos.x + player.radius > area.raw_area.w + 8.0 * 32.0
            && self.get_next_area(&player)
          {
            changes.insert(*id, Change::NextArea);
          }

          if player.area > 0
            && player.pos.x - player.radius < -8.0 * 32.0
            && self.get_prev_area(&player)
          {
            changes.insert(*id, Change::PrevArea);
          }

          if player.area == 0
            && player.pos.x - player.radius < 0.0
            && player.pos.y - player.radius < 2.0 * 32.0
          {
            changes.insert(*id, Change::NextWorld);
          }

          if player.area == 0
            && player.pos.x - player.radius < 0.0
            && player.pos.y + player.radius > area.raw_area.h - 2.0 * 32.0
          {
            changes.insert(*id, Change::PrevWorld);
          }
        }
      }
    }

    changes
  }

  pub fn process_warps(
    &mut self,
    players_manager: &mut PlayersManager,
    config: &Config,
    network_bus: &mut NetworkBus,
  ) {
    let warps = self.prepare_warps(&players_manager.players);
    for (id, change) in &warps {
      if let Some(hero) = players_manager.players.get_mut(&id) {
        let player = hero.player_mut();
        match change {
          Change::NextArea => {
            if let Some(world) = self.worlds.get_mut(&player.world) {
              if let Some(area) = world.areas.get_mut(player.area as usize) {
                area.leave(player.id);
              }
              player.area += 1;
              player.pos.x = -8.0 * 32.0 + player.radius;
              let next_area = world.areas.get_mut(player.area as usize).unwrap();
              next_area.join(player.id);
              let area_init_package = Package::AreaInit(world.pack_area(player.area as usize));
              network_bus.add_direct_package(*id, area_init_package);
              let players_package = Package::Players(players_manager.pack_players());
              network_bus.add_direct_package(*id, players_package);
            }
          }
          Change::PrevArea => {
            if let Some(world) = self.worlds.get_mut(&player.world) {
              if let Some(area) = world.areas.get_mut(player.area as usize) {
                area.leave(player.id);
              }
              player.area -= 1;
              let prev_area = world.areas.get_mut(player.area as usize).unwrap();
              prev_area.join(player.id);

              player.pos.x = prev_area.raw_area.w + 8.0 * 32.0 - player.radius;
              let area_init_package = Package::AreaInit(world.pack_area(player.area as usize));
              network_bus.add_direct_package(*id, area_init_package);
              let players_package = Package::Players(players_manager.pack_players());
              network_bus.add_direct_package(*id, players_package);
            }
          }
          Change::NextWorld => {
            if let Some(prev_world) = self.worlds.get_mut(&player.world) {
              prev_world.leave(&player);
            }
            let next_world_name = WorldsManager::get_next_world(&config.worlds, &player.world);
            let next_world = self.worlds.get_mut(&next_world_name).unwrap();
            let area = next_world.areas.get_mut(0).unwrap();
            player.world = next_world_name;
            player.pos.y = area.raw_area.h - player.radius - 2.0 * 32.0;
            next_world.join(&player);
            let area_init_package = Package::AreaInit(next_world.pack_area(player.area as usize));
            network_bus.add_direct_package(*id, area_init_package);
            let players_package = Package::Players(players_manager.pack_players());
            network_bus.add_direct_package(*id, players_package);
          }
          Change::PrevWorld => {
            if let Some(prev_world) = self.worlds.get_mut(&player.world) {
              prev_world.leave(&player);
            }
            let prev_world_name = WorldsManager::get_prev_world(&config.worlds, &player.world);
            let prev_world = self.worlds.get_mut(&prev_world_name).unwrap();
            player.world = prev_world_name;
            player.pos.y = player.radius + 2.0 * 32.0;
            prev_world.join(&player);
            let area_init_package = Package::AreaInit(prev_world.pack_area(player.area as usize));
            network_bus.add_direct_package(*id, area_init_package);
            let players_package = Package::Players(players_manager.pack_players());
            network_bus.add_direct_package(*id, players_package);
          }
        };
      }
    }
  }

  fn get_next_world(world_names: &Vec<String>, current_world: &String) -> String {
    let current_index = world_names.iter().position(|name| name == current_world);

    match current_index {
      Some(idx) if idx + 1 < world_names.len() => world_names[idx + 1].clone(),
      _ => world_names.get(0).unwrap().clone(),
    }
  }

  fn get_prev_world(world_names: &Vec<String>, current_world: &String) -> String {
    let current_index = world_names.iter().position(|name| name == current_world);

    match current_index {
      Some(idx) if idx > 0 => world_names[idx - 1].clone(),
      _ => world_names.get(world_names.len() - 1).unwrap().clone(),
    }
  }

  pub fn get_next_area(&self, player: &Player) -> bool {
    if let Some(world) = self.worlds.get(&player.world) {
      if let Some(_) = world.areas.get(player.area as usize + 1) {
        return true;
      }
    }
    false
  }

  pub fn get_prev_area(&self, player: &Player) -> bool {
    if let Some(world) = self.worlds.get(&player.world) {
      if let Some(_) = world.areas.get(player.area as usize - 1) {
        return true;
      }
    }
    false
  }
}
