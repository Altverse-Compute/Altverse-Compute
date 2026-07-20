use crate::bus::Client;
use crate::fbs::Package::{self as OwnPackage};
use crate::managers::player::PlayersManager;
use crate::managers::world::WorldsManager;
use crate::pulse_gen::{
  Chat, CloseEntities, ClosePlayer, Entities, Package, Packages, PackedArea, PackedEntity,
  PackedPlayer, PartialEntity, PartialPlayer, Players, UpdateEntities, UpdatePlayers,
};
use crate::resources::area::Area;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::EntityField;
use crate::resources::player::PlayerField;

fn packaged_player(hero_wrapper: &HeroWrapper) -> PackedPlayer {
  let player = hero_wrapper.player();
  PackedPlayer {
    id: player.id,
    name: player.name.clone(),
    x: player.pos.x,
    y: player.pos.y,
    radius: player.radius,
    speed: player.speed,
    energy: player.energy,
    max_energy: player.max_energy,
    death_timer: player.death_timer,
    state: player.state,
    state_meta: player.state_meta,
    area: player.area as u32,
    world: player.world.clone(),
    downed: player.downed,
    hero: player.hero,
  }
}

fn partial_player(hero_wrapper: &HeroWrapper) -> PartialPlayer {
  let player = hero_wrapper.player();
  let mask = player.changes;
  PartialPlayer {
    id: player.id,
    name: if mask & PlayerField::Name as u32 != 0 {
      Some(player.name.clone())
    } else {
      None
    },
    x: if mask & PlayerField::Pos as u32 != 0 {
      Some(player.pos.x)
    } else {
      None
    },
    y: if mask & PlayerField::Pos as u32 != 0 {
      Some(player.pos.y)
    } else {
      None
    },
    radius: if mask & PlayerField::Radius as u32 != 0 {
      Some(player.radius)
    } else {
      None
    },
    speed: if mask & PlayerField::Speed as u32 != 0 {
      Some(player.speed)
    } else {
      None
    },
    energy: if mask & PlayerField::Energy as u32 != 0 {
      Some(player.energy)
    } else {
      None
    },
    max_energy: if mask & PlayerField::MaxEnergy as u32 != 0 {
      Some(player.max_energy)
    } else {
      None
    },
    death_timer: if mask & PlayerField::DeathTimer as u32 != 0 {
      Some(player.death_timer)
    } else {
      None
    },
    state: if mask & PlayerField::State as u32 != 0 {
      Some(player.state)
    } else {
      None
    },
    state_meta: if mask & PlayerField::StateMeta as u32 != 0 {
      Some(player.state_meta)
    } else {
      None
    },
    area: if mask & PlayerField::Area as u32 != 0 {
      Some(player.area as u32)
    } else {
      None
    },
    world: if mask & PlayerField::World as u32 != 0 {
      Some(player.world.clone())
    } else {
      None
    },
    downed: if mask & PlayerField::Downed as u32 != 0 {
      Some(player.downed)
    } else {
      None
    },
  }
}

fn packaged_entity(entity_wrapper: &EntityWrapper) -> PackedEntity {
  let entity = entity_wrapper.entity();
  PackedEntity {
    id: entity.id,
    type_id: entity.type_id as u32,
    x: entity.pos.x,
    y: entity.pos.y,
    radius: entity.radius,
    harmless: entity.harmless,
    state: entity.state,
    state_meta: entity.state_metadata,
    alpha: entity.alpha,
  }
}

fn partial_entity(entity_wrapper: &EntityWrapper) -> PartialEntity {
  let entity = entity_wrapper.entity();
  let mask = entity.changes;
  PartialEntity {
    id: entity.id,
    x: if mask & EntityField::Pos as u8 != 0 {
      Some(entity.pos.x)
    } else {
      None
    },
    y: if mask & EntityField::Pos as u8 != 0 {
      Some(entity.pos.y)
    } else {
      None
    },
    radius: if mask & EntityField::Radius as u8 != 0 {
      Some(entity.radius)
    } else {
      None
    },
    harmless: if mask & EntityField::Harmless as u8 != 0 {
      Some(entity.harmless)
    } else {
      None
    },
    state: if mask & EntityField::State as u8 != 0 {
      Some(entity.state)
    } else {
      None
    },
    state_meta: if mask & EntityField::StateMetadata as u8 != 0 {
      Some(entity.state_metadata)
    } else {
      None
    },
    alpha: if mask & EntityField::Alpha as u8 != 0 {
      Some(entity.alpha)
    } else {
      None
    },
  }
}

fn area_init(area: &Area, world_name: String, area_id: u32) -> PackedArea {
  PackedArea {
    w: area.raw_area.w,
    h: area.raw_area.h,
    area: area_id,
    world: world_name.clone(),
    entities: area.entities.iter().map(|f| packaged_entity(f.1)).collect(),
  }
}

pub fn build_packages<'a>(
  client: &mut Client,
  players_manager: &PlayersManager,
  worlds_manager: &WorldsManager,
) {
  let builder = &mut client.builder;
  let mut packages = Vec::<Package>::new();
  for package in client.packages.clone() {
    match package {
      OwnPackage::NewPlayer(id) => match players_manager.get_player(id) {
        Some(player) => {
          packages.push(Package {
            new_player: Some(packaged_player(player)),
            close_player: None,
            players: None,
            new_entities: None,
            close_entities: None,
            area_init: None,
            myself: None,
            update_entities: None,
            update_players: None,
            chat: None,
          });
        }
        None => {}
      },
      OwnPackage::ClosePlayer(id) => packages.push(Package {
        new_player: None,
        close_player: Some(ClosePlayer { id: id as u32 }),
        players: None,
        new_entities: None,
        close_entities: None,
        area_init: None,
        myself: None,
        update_entities: None,
        update_players: None,
        chat: None,
      }),
      OwnPackage::Players(ids) => {
        let mut packed_players = Vec::<PackedPlayer>::new();
        for id in ids {
          if let Some(player) = players_manager.get_player(id) {
            packed_players.push(packaged_player(player));
          }
        }
        packages.push(Package {
          new_player: None,
          close_player: None,
          players: Some(Players {
            players: packed_players,
          }),
          new_entities: None,
          close_entities: None,
          area_init: None,
          myself: None,
          update_entities: None,
          update_players: None,
          chat: None,
        });
      }
      OwnPackage::UpdatePlayers(ids) => {
        let mut partial_players = Vec::<PartialPlayer>::new();
        for id in ids {
          if let Some(player) = players_manager.get_player(id) {
            partial_players.push(partial_player(player));
          }
        }
        packages.push(Package {
          new_player: None,
          close_player: None,
          players: None,
          new_entities: None,
          close_entities: None,
          area_init: None,
          myself: None,
          update_entities: None,
          update_players: Some(UpdatePlayers {
            items: partial_players,
          }),
          chat: None,
        });
      }
      OwnPackage::Myself(id) => {
        if let Some(player) = players_manager.get_player(id) {
          packages.push(Package {
            new_player: None,
            close_player: None,
            players: None,
            new_entities: None,
            close_entities: None,
            area_init: None,
            myself: Some(packaged_player(player)),
            update_entities: None,
            update_players: None,
            chat: None,
          });
        }
      }
      OwnPackage::NewEntities((ids, world, area)) => {
        let mut packed_entities = Vec::<PackedEntity>::new();
        let area = worlds_manager
          .worlds
          .get(&world)
          .unwrap()
          .areas
          .get(area)
          .unwrap();
        for id in ids {
          if let Some(entity) = area.entities.get(&id) {
            packed_entities.push(packaged_entity(entity));
          }
        }
        packages.push(Package {
          new_player: None,
          close_player: None,
          players: None,
          new_entities: Some(Entities {
            entities: packed_entities,
          }),
          close_entities: None,
          area_init: None,
          myself: None,
          update_entities: None,
          update_players: None,
          chat: None,
        });
      }
      OwnPackage::UpdateEntities((ids, world, area)) => {
        let mut partial_entities = Vec::<PartialEntity>::new();
        let area = worlds_manager
          .worlds
          .get(&world)
          .unwrap()
          .areas
          .get(area)
          .unwrap();
        for id in ids {
          if let Some(entity) = area.entities.get(&id) {
            partial_entities.push(partial_entity(entity));
          }
        }
        packages.push(Package {
          new_player: None,
          close_player: None,
          players: None,
          new_entities: None,
          close_entities: None,
          area_init: None,
          myself: None,
          update_entities: Some(UpdateEntities {
            items: partial_entities,
          }),
          update_players: None,
          chat: None,
        });
      }
      OwnPackage::CloseEntities(ids) => {
        packages.push(Package {
          new_player: None,
          close_player: None,
          players: None,
          new_entities: None,
          close_entities: Some(CloseEntities {
            ids: ids.iter().map(|f| *f as u32).collect(),
          }),
          area_init: None,
          myself: None,
          update_entities: None,
          update_players: None,
          chat: None,
        });
      }
      OwnPackage::AreaInit(area) => {
        let init = area_init(
          worlds_manager
            .worlds
            .get(&area.world)
            .unwrap()
            .areas
            .get(area.area as usize)
            .unwrap(),
          area.world,
          area.area,
        );

        packages.push(Package {
          new_player: None,
          close_player: None,
          players: None,
          new_entities: None,
          close_entities: None,
          area_init: Some(init),
          myself: None,
          update_entities: None,
          update_players: None,
          chat: None,
        });
      }
      OwnPackage::Chat(message) => {
        let chat = Chat {
          id: message.id,
          content: message.content,
          author: message.author,
          world: message.world,
        };

        packages.push(Package {
          new_player: None,
          close_player: None,
          players: None,
          new_entities: None,
          close_entities: None,
          area_init: None,
          myself: None,
          update_entities: None,
          update_players: None,
          chat: Some(chat),
        });
      }
    }
  }

  Packages::write_package(&Packages { items: packages }, builder);
}
