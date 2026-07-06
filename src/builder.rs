use crate::bus::Client;
use crate::fbs::{Chat as OwnChat, Package as OwnPackage, PackedArea as OwnPackedArea};
use crate::flat::altverse_server::{
  Chat, ChatArgs, CloseEntities, CloseEntitiesArgs, ClosePlayer, ClosePlayerArgs, Entities,
  EntitiesArgs, Package, PackageArgs, PackageKind, Packages, PackagesArgs, PackedArea,
  PackedAreaArgs, PackedEntity, PackedEntityArgs, PackedEntityMap, PackedEntityMapArgs,
  PackedPlayer, PackedPlayerArgs, PackedPlayerMap, PackedPlayerMapArgs, PartialEntity,
  PartialEntityArgs, PartialEntityMap, PartialEntityMapArgs, PartialPlayer, PartialPlayerArgs,
  PartialPlayerMap, PartialPlayerMapArgs, Players, PlayersArgs, Role, UpdateEntities,
  UpdateEntitiesArgs, UpdatePlayers, UpdatePlayersArgs,
};
use crate::managers::player::PlayersManager;
use crate::managers::world::WorldsManager;
use crate::resources::area::Area;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::EntityField;
use crate::resources::player::PlayerField;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

fn build_packed_player<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  hero_wrapper: &HeroWrapper,
) -> WIPOffset<PackedPlayer<'a>> {
  let player = hero_wrapper.player();
  let name = builder.create_string(player.name.as_str());
  let world = builder.create_string(player.world.as_str());
  PackedPlayer::create(
    builder,
    &PackedPlayerArgs {
      name: Some(name),
      id: player.id,
      x: player.pos.x,
      y: player.pos.y,
      radius: player.radius,
      speed: player.speed,
      energy: player.energy,
      max_energy: player.max_energy,
      death_timer: player.death_timer,
      state: player.state,
      state_meta: player.state_meta,
      area: player.area,
      world: Some(world),
      died: player.downed,
      hero: player.hero,
    },
  )
}

pub fn build_new_player<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  hero_wrapper: &HeroWrapper,
) -> WIPOffset<Package<'a>> {
  let kind = build_packed_player(builder, hero_wrapper);
  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::new_player,
      kind: Some(kind.as_union_value()),
    },
  )
}

pub fn build_new_entity<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  hero_wrapper: &EntityWrapper,
) -> WIPOffset<PackedEntity<'a>> {
  let entity = hero_wrapper.entity();

  PackedEntity::create(
    builder,
    &PackedEntityArgs {
      type_id: entity.type_id,
      x: entity.pos.x,
      y: entity.pos.y,
      radius: entity.radius,
      harmless: entity.harmless,
      state: entity.state,
      state_metadata: entity.state_metadata,
      alpha: entity.alpha,
    },
  )
}

fn build_close_player<'a>(builder: &mut FlatBufferBuilder<'a>, id: u64) -> WIPOffset<Package<'a>> {
  let close_player = ClosePlayer::create(builder, &ClosePlayerArgs { id: id });

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::close_player,
      kind: Some(close_player.as_union_value()),
    },
  )
}

fn build_partial_entity<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  wrapper: &EntityWrapper,
) -> WIPOffset<PartialEntity<'a>> {
  let entity = wrapper.entity();
  let changes = entity.get_changes();
  PartialEntity::create(
    builder,
    &PartialEntityArgs {
      x: changes.contains(&EntityField::Pos).then_some(entity.pos.x),
      y: changes.contains(&EntityField::Pos).then_some(entity.pos.y),
      radius: changes
        .contains(&EntityField::Radius)
        .then_some(entity.radius),
      harmless: changes
        .contains(&EntityField::Harmless)
        .then_some(entity.harmless),
      state: changes
        .contains(&EntityField::State)
        .then_some(entity.state),
      state_metadata: changes
        .contains(&EntityField::StateMetadata)
        .then_some(entity.state_metadata),
      alpha: changes
        .contains(&EntityField::Alpha)
        .then_some(entity.alpha),
    },
  )
}

fn build_partial_player<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  wrapper: &HeroWrapper,
) -> WIPOffset<PartialPlayer<'a>> {
  let player = wrapper.player();
  let changes = player.get_changes();
  let mut world = if changes.contains(&PlayerField::World) {
    Some(builder.create_string(player.world.as_str()))
  } else {
    None
  };

  PartialPlayer::create(
    builder,
    &PartialPlayerArgs {
      x: changes.contains(&PlayerField::Pos).then_some(player.pos.x),
      y: changes.contains(&PlayerField::Pos).then_some(player.pos.y),
      radius: changes
        .contains(&PlayerField::Radius)
        .then_some(player.radius),
      speed: changes
        .contains(&PlayerField::Speed)
        .then_some(player.speed),
      energy: changes
        .contains(&PlayerField::Energy)
        .then_some(player.energy),
      max_energy: changes
        .contains(&PlayerField::MaxEnergy)
        .then_some(player.max_energy),
      death_timer: changes
        .contains(&PlayerField::DeathTimer)
        .then_some(player.death_timer),
      state: changes
        .contains(&PlayerField::State)
        .then_some(player.state),
      state_metadata: changes
        .contains(&PlayerField::StateMeta)
        .then_some(player.state_meta),
      area: changes.contains(&PlayerField::Area).then_some(player.area),
      world,
      died: changes
        .contains(&PlayerField::Downed)
        .then_some(player.downed),
    },
  )
}

fn build_players<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  players_manager: &PlayersManager,
  ids: Vec<u64>,
  kind_type: PackageKind,
) -> WIPOffset<Package<'a>> {
  let mut players = Vec::new();
  for id in ids {
    match players_manager.get_player(id) {
      Some(player) => {
        let builded_partial = build_packed_player(builder, player);
        let pack = PackedPlayerMap::create(
          builder,
          &PackedPlayerMapArgs {
            key: id,
            value: Some(builded_partial),
          },
        );
        players.push(pack);
      }
      None => {}
    }
  }
  let partials = builder.create_vector(&players);

  let update_players = Players::create(
    builder,
    &PlayersArgs {
      players: Some(partials),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type,
      kind: Some(update_players.as_union_value()),
    },
  )
}

fn build_update_players<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  players_manager: &PlayersManager,
  ids: Vec<u64>,
) -> WIPOffset<Package<'a>> {
  let mut players = Vec::new();
  for id in ids {
    match players_manager.get_player(id) {
      Some(player) => {
        let builded_partial = build_partial_player(builder, player);
        let pack = PartialPlayerMap::create(
          builder,
          &PartialPlayerMapArgs {
            key: id,
            value: Some(builded_partial),
          },
        );
        players.push(pack);
      }
      None => {}
    }
  }
  let partials = builder.create_vector(&players);

  let update_players = UpdatePlayers::create(
    builder,
    &UpdatePlayersArgs {
      items: Some(partials),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::update_players,
      kind: Some(update_players.as_union_value()),
    },
  )
}

fn build_myself<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  player: &HeroWrapper,
) -> WIPOffset<Package<'a>> {
  let packed_player = build_packed_player(builder, player);

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::myself,
      kind: Some(packed_player.as_union_value()),
    },
  )
}

fn build_new_entities<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  area: &Area,
  ids: Vec<u64>,
) -> WIPOffset<Package<'a>> {
  let mut players = Vec::new();
  for id in ids {
    match area.entities.get(&id) {
      Some(entity) => {
        let builded_partial = build_new_entity(builder, entity);
        let pack = PackedEntityMap::create(
          builder,
          &PackedEntityMapArgs {
            key: id,
            value: Some(builded_partial),
          },
        );
        players.push(pack);
      }
      None => {}
    }
  }
  let partials = builder.create_vector(&players);

  let update_players = Entities::create(
    builder,
    &EntitiesArgs {
      entities: Some(partials),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::new_entities,
      kind: Some(update_players.as_union_value()),
    },
  )
}

fn build_update_entities<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  area: &Area,
  ids: Vec<u64>,
) -> WIPOffset<Package<'a>> {
  let mut players = Vec::new();
  for id in ids {
    match area.entities.get(&id) {
      Some(entity) => {
        let builded_partial = build_partial_entity(builder, entity);
        let pack = PartialEntityMap::create(
          builder,
          &PartialEntityMapArgs {
            key: id,
            value: Some(builded_partial),
          },
        );
        players.push(pack);
      }
      None => {}
    }
  }
  let partials = builder.create_vector(&players);

  let update_players = UpdateEntities::create(
    builder,
    &UpdateEntitiesArgs {
      items: Some(partials),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::update_entities,
      kind: Some(update_players.as_union_value()),
    },
  )
}

fn build_close_entities<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  ids: Vec<u64>,
) -> WIPOffset<Package<'a>> {
  let partials = builder.create_vector(&ids);

  let close_entities = CloseEntities::create(
    builder,
    &CloseEntitiesArgs {
      ids: Some(partials),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::close_entities,
      kind: Some(close_entities.as_union_value()),
    },
  )
}

fn build_area_init<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  area: &Area,
  area_props: &OwnPackedArea,
) -> WIPOffset<Package<'a>> {
  let mut players = Vec::new();
  let ids = area_props.entities.clone();
  for id in ids {
    match area.entities.get(&id) {
      Some(entity) => {
        let builded_partial = build_new_entity(builder, entity);
        let pack = PackedEntityMap::create(
          builder,
          &PackedEntityMapArgs {
            key: id,
            value: Some(builded_partial),
          },
        );
        players.push(pack);
      }
      None => {}
    }
  }

  let world = builder.create_string(area_props.world.as_str());
  let partials = builder.create_vector(&players);

  let update_players = PackedArea::create(
    builder,
    &PackedAreaArgs {
      w: area_props.w,
      h: area_props.h,
      area: area_props.area,
      world: Some(world),
      entities: Some(partials),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::area_init,
      kind: Some(update_players.as_union_value()),
    },
  )
}

fn build_chat_message<'a>(
  builder: &mut FlatBufferBuilder<'a>,
  message: &OwnChat,
) -> WIPOffset<Package<'a>> {
  let content = builder.create_string(message.content.as_str());
  let author = builder.create_string(message.author.as_str());
  let world = builder.create_string(message.world.as_str());
  let message = Chat::create(
    builder,
    &ChatArgs {
      id: message.id,
      content: Some(content),
      author: Some(author),
      role: Role::User,
      world: Some(world),
    },
  );

  Package::create(
    builder,
    &PackageArgs {
      kind_type: PackageKind::chat,
      kind: Some(message.as_union_value()),
    },
  )
}

pub fn build_packages<'a>(
  client: &mut Client,
  players_manager: &PlayersManager,
  worlds_manager: &WorldsManager,
) -> WIPOffset<Packages<'a>> {
  let mut packages = Vec::new();
  for package in client.packages.clone() {
    match package {
      OwnPackage::NewPlayer(id) => match players_manager.get_player(id) {
        Some(player) => {
          packages.push(build_new_player(&mut client.flat_builder, player));
        }
        None => {}
      },
      OwnPackage::ClosePlayer(id) => match players_manager.get_player(id) {
        Some(player) => {
          packages.push(build_close_player(
            &mut client.flat_builder,
            player.player().id,
          ));
        }
        None => {}
      },
      OwnPackage::Players(ids) => {
        packages.push(build_players(
          &mut client.flat_builder,
          players_manager,
          ids,
          PackageKind::players,
        ));
      }
      OwnPackage::UpdatePlayers(ids) => {
        packages.push(build_update_players(
          &mut client.flat_builder,
          players_manager,
          ids,
        ));
      }
      OwnPackage::Myself(id) => {
        packages.push(build_myself(
          &mut client.flat_builder,
          players_manager.get_player(id).unwrap(),
        ));
      }
      OwnPackage::NewEntities((ids, world, area)) => packages.push(build_new_entities(
        &mut client.flat_builder,
        worlds_manager
          .worlds
          .get(&world)
          .unwrap()
          .areas
          .get(area)
          .unwrap(),
        ids,
      )),
      OwnPackage::UpdateEntities((ids, world, area)) => packages.push(build_update_entities(
        &mut client.flat_builder,
        worlds_manager
          .worlds
          .get(&world)
          .unwrap()
          .areas
          .get(area)
          .unwrap(),
        ids,
      )),
      OwnPackage::CloseEntities(ids) => {
        packages.push(build_close_entities(&mut client.flat_builder, ids))
      }
      OwnPackage::AreaInit(area) => packages.push(build_area_init(
        &mut client.flat_builder,
        worlds_manager
          .worlds
          .get(&area.world)
          .unwrap()
          .areas
          .get(area.area as usize)
          .unwrap(),
        &area,
      )),
      OwnPackage::Chat(message) => {
        packages.push(build_chat_message(&mut client.flat_builder, &message))
      }
    }
  }

  let items = client.flat_builder.create_vector(&packages);

  Packages::create(
    &mut client.flat_builder,
    &PackagesArgs { items: Some(items) },
  )
}
