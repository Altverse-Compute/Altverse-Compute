use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
  fbs::{Chat, PackedArea},
  managers::player::PlayersManager,
  resources::{
    area::Area,
    assets::{entity::EntityWrapper, hero::HeroWrapper},
    entity::EntityField,
    player::PlayerField,
  },
};

pub struct ByteWriter {
  data: Vec<u8>,
}

impl ByteWriter {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      data: Vec::with_capacity(capacity),
    }
  }

  pub fn write_u8(&mut self, value: u8) {
    self.data.push(value);
  }

  pub fn write_bool(&mut self, value: bool) {
    self.data.push(value as u8);
  }

  pub fn write_u16(&mut self, value: u16) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_u32(&mut self, value: u32) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_u64(&mut self, value: u64) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_i16(&mut self, value: i16) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_i32(&mut self, value: i32) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_f32(&mut self, value: f32) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_var_u32(&mut self, mut value: u32) {
    while value >= 0x80 {
      self.data.push((value as u8) | 0x80);
      value >>= 7;
    }

    self.data.push(value as u8);
  }

  pub fn write_bytes(&mut self, bytes: &[u8]) {
    self.data.extend_from_slice(bytes);
  }

  pub fn into_inner(self) -> Vec<u8> {
    self.data
  }

  pub fn as_slice(&self) -> &[u8] {
    &self.data
  }

  pub fn clear(&mut self) {
    self.data.clear();
  }
}

#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive)]
enum PackageHeader {
  ChatMessage = 0,
  NewPlayer,
  ClosePlayer,
  Players,
  NewEntities,
  CloseEntities,
  AreaInit,
  MySelf,
  UpdateEntities,
  UpdatePlayers,
}

pub struct PulseBuilder {
  pub bytes: ByteWriter,
}

impl PulseBuilder {
  pub fn new() -> Self {
    Self {
      bytes: ByteWriter::new(),
    }
  }

  pub fn clean(&mut self) {
    self.bytes.data.clear();
  }

  fn write_header(&mut self, package_header: PackageHeader) {
    self.bytes.write_u8(package_header.into());
  }

  fn write_array_header(&mut self, length: usize) {
    self.bytes.write_var_u32(length as u32);
  }

  fn write_packed_player(&mut self, hero_wrapper: &HeroWrapper) {
    let bytes = &mut self.bytes;
    let player = hero_wrapper.player();

    bytes.write_var_u32(player.id as u32);
    bytes.write_var_u32(player.name.len() as u32);
    bytes.write_bytes(player.name.as_bytes());
    bytes.write_f32(player.pos.x);
    bytes.write_f32(player.pos.y);
    bytes.write_f32(player.radius);
    bytes.write_f32(player.speed);
    bytes.write_f32(player.energy);
    bytes.write_f32(player.max_energy);
    bytes.write_f32(player.death_timer);
    bytes.write_u8(player.state);
    bytes.write_f32(player.state_meta);
    bytes.write_var_u32(player.area as u32);
    bytes.write_var_u32(player.world.len() as u32);
    bytes.write_bytes(player.world.as_bytes());
    bytes.write_bool(player.downed);
    bytes.write_u32(player.hero);
  }

  fn write_partial_player(&mut self, hero_wrapper: &HeroWrapper) {
    let bytes = &mut self.bytes;
    let player = hero_wrapper.player();
    let mask = player.get_changes();

    bytes.write_u32(mask);
    bytes.write_var_u32(player.id as u32);
    if mask & PlayerField::Pos as u32 != 0 {
      bytes.write_u16((player.pos.x * 2f32) as u16);
      bytes.write_u16((player.pos.y * 2f32) as u16);
    }
    if mask & PlayerField::Radius as u32 != 0 {
      bytes.write_u16((player.radius * 2f32) as u16);
    }
    if mask & PlayerField::Speed as u32 != 0 {
      bytes.write_u16((player.speed * 2f32) as u16);
    }
    if mask & PlayerField::Energy as u32 != 0 {
      bytes.write_u16((player.energy * 2f32) as u16);
    }
    if mask & PlayerField::MaxEnergy as u32 != 0 {
      bytes.write_u16((player.max_energy * 2f32) as u16);
    }
    if mask & PlayerField::DeathTimer as u32 != 0 {
      bytes.write_u8((player.death_timer * 60f32) as u8);
    }
    if mask & PlayerField::State as u32 != 0 {
      bytes.write_u8(player.state);
    }
    if mask & PlayerField::StateMeta as u32 != 0 {
      bytes.write_f32(player.state_meta);
    }
    if mask & PlayerField::Area as u32 != 0 {
      bytes.write_var_u32(player.area as u32);
    }
    if mask & PlayerField::World as u32 != 0 {
      bytes.write_var_u32(player.world.len() as u32);
      bytes.write_bytes(player.world.as_bytes());
    }
    if mask & PlayerField::Downed as u32 != 0 {
      bytes.write_bool(player.downed);
    }
  }

  fn write_packed_entity(&mut self, entity_wrapper: &EntityWrapper) {
    let bytes = &mut self.bytes;
    let entity = entity_wrapper.entity();

    bytes.write_var_u32(entity.id as u32);
    bytes.write_var_u32(entity.type_id as u32);
    bytes.write_f32(entity.pos.x);
    bytes.write_f32(entity.pos.y);
    bytes.write_f32(entity.radius);
    bytes.write_bool(entity.harmless);
    bytes.write_u8(entity.state);
    bytes.write_f32(entity.state_metadata);
    bytes.write_f32(entity.alpha);
  }

  fn write_partial_entity(&mut self, entity_wrapper: &EntityWrapper) {
    let bytes = &mut self.bytes;
    let entity = entity_wrapper.entity();
    let mask = entity.get_changes();

    bytes.write_u8(mask);
    bytes.write_var_u32(entity.id as u32);
    if mask & EntityField::Pos as u8 != 0 {
      bytes.write_u16((entity.pos.x * 2f32) as u16);
      bytes.write_u16((entity.pos.y * 2f32) as u16);
    }
    if mask & EntityField::Radius as u8 != 0 {
      bytes.write_u16((entity.radius * 2f32) as u16);
    }
    if mask & EntityField::Harmless as u8 != 0 {
      bytes.write_bool(entity.harmless);
    }
    if mask & EntityField::State as u8 != 0 {
      bytes.write_u8(entity.state);
    }
    if mask & EntityField::StateMetadata as u8 != 0 {
      bytes.write_u16((entity.state_metadata * 2f32) as u16);
    }
    if mask & EntityField::Alpha as u8 != 0 {
      bytes.write_u8((entity.alpha * 255f32) as u8);
    }
  }

  pub fn write_new_player(&mut self, hero_wrapper: &HeroWrapper) {
    self.write_header(PackageHeader::NewPlayer);
    self.write_packed_player(hero_wrapper);
  }

  pub fn write_close_player(&mut self, id: u64) {
    self.write_header(PackageHeader::ClosePlayer);
    self.bytes.write_var_u32(id as u32);
  }

  pub fn write_players(&mut self, players_manager: &PlayersManager, ids: Vec<u64>) {
    self.write_header(PackageHeader::Players);
    self.write_array_header(ids.len());
    for id in ids {
      let hero_wrapper = players_manager.get_player(id);
      if let Some(hero_wrapper) = hero_wrapper {
        self.write_packed_player(hero_wrapper);
      }
    }
  }

  pub fn write_new_entities(&mut self, area: &Area, ids: Vec<u64>) {
    self.write_header(PackageHeader::NewEntities);
    self.write_array_header(ids.len());
    for id in ids {
      let entity_wrapper = area.entities.get(&id);
      if let Some(entity_wrapper) = entity_wrapper {
        self.write_packed_entity(entity_wrapper);
      }
    }
  }

  pub fn write_close_entities(&mut self, ids: Vec<u64>) {
    self.write_header(PackageHeader::CloseEntities);
    self.write_array_header(ids.len());
    for id in ids {
      self.bytes.write_var_u32(id as u32);
    }
  }

  pub fn write_update_entities(&mut self, area: &Area, ids: Vec<u64>) {
    self.write_header(PackageHeader::UpdateEntities);
    self.write_array_header(ids.len());
    for id in ids {
      let entity_wrapper = area.entities.get(&id);
      if let Some(entity_wrapper) = entity_wrapper {
        self.write_partial_entity(entity_wrapper);
      }
    }
  }

  pub fn write_area_init(&mut self, area: &Area, area_props: &PackedArea) {
    self.write_header(PackageHeader::AreaInit);
    self.bytes.write_f32(area.raw_area.w);
    self.bytes.write_f32(area.raw_area.h);
    self.bytes.write_var_u32(area_props.area);
    self.bytes.write_var_u32(area_props.world.len() as u32);
    self.bytes.write_bytes(area_props.world.as_bytes());
    let ids = area.get_packed_entities();
    self.write_array_header(ids.len());
    for id in ids {
      if let Some(entity_wrapper) = area.entities.get(&id) {
        self.write_packed_entity(entity_wrapper);
      }
    }
  }

  pub fn write_my_self(&mut self, hero_wrapper: &HeroWrapper) {
    self.write_header(PackageHeader::MySelf);
    self.write_packed_player(hero_wrapper);
  }

  pub fn write_update_players(&mut self, players_manager: &PlayersManager, ids: Vec<u64>) {
    self.write_header(PackageHeader::UpdatePlayers);
    self.write_array_header(ids.len());
    for id in ids {
      if let Some(entity_wrapper) = players_manager.players.get(&id) {
        self.write_partial_player(entity_wrapper);
      }
    }
  }

  pub fn write_chat_message(&mut self, chat_message: &Chat) {
    self.write_header(PackageHeader::ChatMessage);
    self.bytes.write_u64(chat_message.id);
    self.bytes.write_var_u32(chat_message.content.len() as u32);
    self.bytes.write_bytes(chat_message.content.as_bytes());
    self.bytes.write_var_u32(chat_message.author.len() as u32);
    self.bytes.write_bytes(chat_message.author.as_bytes());
    self.bytes.write_var_u32(chat_message.world.len() as u32);
    self.bytes.write_bytes(chat_message.world.as_bytes());
  }
}
