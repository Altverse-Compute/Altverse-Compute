use crate::{
  bus::NetworkBus,
  external::{Chat, Package, Role},
  managers::{player::PlayersManager, world::WorldsManager},
};

pub struct ChatManager {}

impl ChatManager {
  pub fn message(
    network_bus: &mut NetworkBus,
    players_manager: &mut PlayersManager,
    worlds_manager: &mut WorldsManager,
    content: String,
    id: i64,
  ) {
    let packed_players = players_manager.pack_players();
    if let Some(hero) = players_manager.get_mut_player(id as u64) {
      let lowercase_content = content.to_lowercase();
      let parts: Vec<&str> = lowercase_content.split(" ").collect::<Vec<&str>>();
      if parts[0] == "/res" {
        match hero.player().role {
          Role::Dev => {}
          _ => {
            network_bus.add_direct_package(
              id as u64,
              Package::Chat(Chat {
                id: id.try_into().unwrap(),
                content: "You are not developer.".into(),
                author: "".into(),
                role: Role::Server,
                world: hero.player().world.clone(),
              }),
            );
            return;
          }
        }
        hero.res();
        network_bus.add_direct_package(
          id as u64,
          Package::Chat(Chat {
            id: id.try_into().unwrap(),
            content: "You are alive!".into(),
            author: "".into(),
            role: Role::Server,
            world: hero.player().world.clone(),
          }),
        );
      } else if parts[0] == "/warp" {
        match hero.player().role {
          Role::Dev => {}
          _ => {
            network_bus.add_direct_package(
              id as u64,
              Package::Chat(Chat {
                id: id.try_into().unwrap(),
                content: "You are not developer.".into(),
                author: "".into(),
                role: Role::Server,
                world: hero.player().world.clone(),
              }),
            );
            return;
          }
        }
        if parts.len() != 2 {
          network_bus.add_direct_package(
            id as u64,
            Package::Chat(Chat {
              id: id.try_into().unwrap(),
              content: "Incorrect command use. Usage: /warp {number}".into(),
              author: "".into(),
              role: Role::Server,
              world: hero.player().world.clone(),
            }),
          );
          return;
        }
        let area = parts[1].parse::<i64>();
        if !area.is_ok() {
          network_bus.add_direct_package(
            id as u64,
            Package::Chat(Chat {
              id: id.try_into().unwrap(),
              content: "Incorrect command use. Usage: /warp {positive number}".into(),
              author: "".into(),
              role: Role::Server,
              world: hero.player().world.clone(),
            }),
          );
          return;
        }
        let area = area.unwrap();
        let player = hero.player_mut();
        worlds_manager.external_warp_next_area(
          &player.id.clone(),
          player,
          packed_players,
          network_bus,
          area,
        );
        network_bus.add_direct_package(
          id as u64,
          Package::Chat(Chat {
            id: id.try_into().unwrap(),
            content: format!("You are warped to {}", area).into(),
            author: "".into(),
            role: Role::Server,
            world: hero.player().world.clone(),
          }),
        );
      } else if parts[0].starts_with("/") {
      } else {
        network_bus.add_global_package(Package::Chat(Chat {
          id: id.try_into().unwrap(),
          content,
          author: hero.player().name.clone(),
          role: hero.player().role.clone(),
          world: hero.player().world.clone(),
        }));
      }
    }
  }
}
