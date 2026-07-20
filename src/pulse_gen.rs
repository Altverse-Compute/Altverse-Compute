pub type PResult<T> = Result<T, String>;

pub struct BufferReader<'a> {
  data: &'a [u8],
  pub offset: usize,
}

impl<'a> BufferReader<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    BufferReader { data, offset: 0 }
  }

  fn ensure(&self, bytes: usize) -> PResult<()> {
    if self.offset + bytes > self.data.len() {
      return Err(format!(
        "BufferReader: Attempt read {} byte outbound of buffer (offset={}, length={})",
        bytes,
        self.offset,
        self.data.len()
      ));
    }
    Ok(())
  }

  pub fn read_u8(&mut self) -> PResult<u8> {
    self.ensure(1)?;
    let v = self.data[self.offset];
    self.offset += 1;
    Ok(v)
  }

  pub fn read_i8(&mut self) -> PResult<i8> {
    self.ensure(1)?;
    let v = self.data[self.offset] as i8;
    self.offset += 1;
    Ok(v)
  }

  pub fn read_bool(&mut self) -> PResult<bool> {
    Ok(self.read_u8()? != 0)
  }

  pub fn read_u16(&mut self) -> PResult<u16> {
    self.ensure(2)?;
    let v = u16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
    self.offset += 2;
    Ok(v)
  }

  pub fn read_i16(&mut self) -> PResult<i16> {
    self.ensure(2)?;
    let v = i16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
    self.offset += 2;
    Ok(v)
  }

  pub fn read_u32(&mut self) -> PResult<u32> {
    self.ensure(4)?;
    let v = u32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
    self.offset += 4;
    Ok(v)
  }

  pub fn read_i32(&mut self) -> PResult<i32> {
    self.ensure(4)?;
    let v = i32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
    self.offset += 4;
    Ok(v)
  }

  pub fn read_var_u32(&mut self) -> PResult<u32> {
    let mut x: u64 = 0;
    let mut shift: u32 = 0;
    loop {
      let b = self.read_u8()? as u64;
      x = x.wrapping_add(b << shift);
      if b & 0x80 == 0 {
        return Ok(x as u32);
      }
      shift += 7;
      if shift > 49 {
        return Err("BufferReader: could not decode varint".to_string());
      }
    }
  }

  pub fn read_var_i32(&mut self) -> PResult<i32> {
    let encoded = self.read_var_u32()?;
    Ok(((encoded >> 1) as i32) ^ -((encoded & 1) as i32))
  }

  pub fn read_u64(&mut self) -> PResult<u64> {
    self.ensure(8)?;
    let v = u64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
    self.offset += 8;
    Ok(v)
  }

  pub fn read_i64(&mut self) -> PResult<i64> {
    self.ensure(8)?;
    let v = i64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
    self.offset += 8;
    Ok(v)
  }

  fn half_bits_to_f32(bits: u16) -> f32 {
    let sign: f32 = if (bits >> 15) != 0 { -1.0 } else { 1.0 };
    let exponent = ((bits & 0x7c00) >> 10) as i32;
    let fraction = (bits & 0x03ff) as f32;

    if exponent == 0 {
      return sign * fraction * 2f32.powi(-24);
    }

    if exponent == 0x1f {
      return if fraction != 0.0 {
        f32::NAN
      } else {
        sign * f32::INFINITY
      };
    }

    sign * (1.0 + fraction / 1024.0) * 2f32.powi(exponent - 15)
  }

  pub fn read_f16(&mut self) -> PResult<f32> {
    let bits = self.read_u16()?;
    Ok(Self::half_bits_to_f32(bits))
  }

  pub fn read_f32(&mut self) -> PResult<f32> {
    self.ensure(4)?;
    let v = f32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
    self.offset += 4;
    Ok(v)
  }

  pub fn read_f64(&mut self) -> PResult<f64> {
    self.ensure(8)?;
    let v = f64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
    self.offset += 8;
    Ok(v)
  }

  pub fn read_char(&mut self) -> PResult<String> {
    self.ensure(1)?;
    let byte = self.data[self.offset];
    self.offset += 1;
    Ok((byte as char).to_string())
  }

  pub fn read_string(&mut self) -> PResult<String> {
    let length = self.read_var_u32()? as usize;
    self.ensure(length)?;
    let bytes = &self.data[self.offset..self.offset + length];
    self.offset += length;
    Ok(String::from_utf8_lossy(bytes).into_owned())
  }

  pub fn read_bytes(&mut self, length: usize) -> PResult<Vec<u8>> {
    self.ensure(length)?;
    let bytes = self.data[self.offset..self.offset + length].to_vec();
    self.offset += length;
    Ok(bytes)
  }

  pub fn remaining(&self) -> usize {
    self.data.len() - self.offset
  }

  pub fn eof(&self) -> bool {
    self.offset >= self.data.len()
  }
}

pub struct BufferWriter {
  data: Vec<u8>,
}

impl BufferWriter {
  pub fn new(initial_capacity: usize) -> Self {
    BufferWriter {
      data: Vec::with_capacity(initial_capacity),
    }
  }

  pub fn write_u8(&mut self, value: u8) {
    self.data.push(value);
  }

  pub fn write_i8(&mut self, value: i8) {
    self.data.push(value as u8);
  }

  pub fn write_bool(&mut self, value: bool) {
    self.write_u8(if value { 1 } else { 0 });
  }

  pub fn write_u16(&mut self, value: u16) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_i16(&mut self, value: i16) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_u32(&mut self, value: u32) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_i32(&mut self, value: i32) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_var_u32(&mut self, value: u32) {
    let mut x = value;
    while x & 0xffffff80 != 0 {
      self.write_u8(((x & 0x7f) | 0x80) as u8);
      x >>= 7;
      x = x.wrapping_sub(1);
    }
    self.write_u8((x & 0x7f) as u8);
  }

  pub fn write_var_i32(&mut self, value: i32) {
    let encoded = ((value << 1) ^ (value >> 31)) as u32;
    self.write_var_u32(encoded);
  }

  pub fn write_u64(&mut self, value: u64) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_i64(&mut self, value: i64) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  fn float_to_half_bits(value: f32) -> u16 {
    let x = value.to_bits() as i32;
    let sign = (x >> 16) & 0x8000;
    let mut m = (x >> 12) & 0x07ff;
    let e = (x >> 23) & 0xff;

    if e < 103 {
      return sign as u16;
    }

    if e > 142 {
      if e == 255 && (x & 0x007fffff) != 0 {
        return (sign | 0x7c00 | 0x0200) as u16;
      }
      return (sign | 0x7c00) as u16;
    }

    if e < 113 {
      m |= 0x0800;
      return (sign | ((m >> (114 - e)) + ((m >> (113 - e)) & 1))) as u16;
    }

    let mut bits = sign | ((e - 112) << 10) | (m >> 1);
    bits += m & 1;
    bits as u16
  }

  pub fn write_f16(&mut self, value: f32) {
    let bits = Self::float_to_half_bits(value);
    self.write_u16(bits);
  }

  pub fn write_f32(&mut self, value: f32) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_f64(&mut self, value: f64) {
    self.data.extend_from_slice(&value.to_le_bytes());
  }

  pub fn write_char(&mut self, value: &str) {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
      panic!(
        "BufferWriter: writeChar char length is below 1 (value = {})",
        value
      );
    }
    self.write_u8(bytes[0]);
  }

  pub fn write_string(&mut self, value: String) {
    let bytes = value.as_bytes();
    self.write_var_u32(bytes.len() as u32);
    self.write_bytes(bytes);
  }

  pub fn write_bytes(&mut self, bytes: &[u8]) {
    self.data.extend_from_slice(bytes);
  }

  pub fn to_vec(&self) -> Vec<u8> {
    self.data.clone()
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn clear(&mut self) {
    self.data.clear();
  }
}

pub struct Quantizer;

impl Quantizer {
  pub fn from_f32_to_q8(value: f32, step: f32) -> i8 {
    let q = (value / step).round();
    q.min(127.0).max(-127.0) as i8
  }

  pub fn from_f32_to_uq8(value: f32, step: f32) -> u8 {
    let q = (value / step).round();
    q.min(255.0).max(0.0) as u8
  }

  pub fn from_q8_to_f32(quantized: i8, step: f32) -> f32 {
    quantized as f32 * step
  }

  pub fn from_f32_to_q16(value: f32, step: f32) -> i16 {
    let q = (value / step).round();
    q.min(32767.0).max(-32767.0) as i16
  }

  pub fn from_f32_to_uq16(value: f32, step: f32) -> u16 {
    let q = (value / step).round();
    q.min(65535.0).max(0.0) as u16
  }

  pub fn from_q16_to_f32(quantized: i16, step: f32) -> f32 {
    quantized as f32 * step
  }
}
#[derive(Debug, Clone)]
pub struct Chat {
  pub id: u64,
  pub content: String,
  pub author: String,
  pub world: String,
}
impl Chat {
  pub fn write_package(value: &Chat, writer: &mut BufferWriter) {
    writer.write_var_u32(1);
    writer.write_var_u32(value.id as u32);
    writer.write_string(value.content.clone());
    writer.write_string(value.author.clone());
    writer.write_string(value.world.clone());
  }
}
#[derive(Debug, Clone)]
pub struct PackedPlayer {
  pub id: u64,
  pub name: String,
  pub x: f32,
  pub y: f32,
  pub radius: f32,
  pub speed: f32,
  pub energy: f32,
  pub max_energy: f32,
  pub death_timer: f32,
  pub state: u8,
  pub state_meta: f32,
  pub area: u32,
  pub world: String,
  pub downed: bool,
  pub hero: u32,
}
impl PackedPlayer {
  pub fn write_package(value: &PackedPlayer, writer: &mut BufferWriter) {
    writer.write_var_u32(2);
    writer.write_var_u32(value.id as u32);
    writer.write_string(value.name.clone());
    writer.write_f32(value.x);
    writer.write_f32(value.y);
    writer.write_f32(value.radius);
    writer.write_f32(value.speed);
    writer.write_f32(value.energy);
    writer.write_f32(value.max_energy);
    writer.write_f32(value.death_timer);
    writer.write_u8(value.state);
    writer.write_f32(value.state_meta);
    writer.write_var_u32(value.area);
    writer.write_string(value.world.clone());
    writer.write_bool(value.downed);
    writer.write_u32(value.hero);
  }
}
#[repr(u32)]
pub enum PartialPlayerBitmask {
  name = 0,
  x = 1,
  y = 2,
  radius = 3,
  speed = 4,
  energy = 5,
  max_energy = 6,
  death_timer = 7,
  state = 8,
  state_meta = 9,
  area = 10,
  world = 11,
  downed = 12,
}
#[derive(Debug, Clone)]
pub struct PartialPlayer {
  pub id: u64,
  pub name: Option<String>,
  pub x: Option<f32>,
  pub y: Option<f32>,
  pub radius: Option<f32>,
  pub speed: Option<f32>,
  pub energy: Option<f32>,
  pub max_energy: Option<f32>,
  pub death_timer: Option<f32>,
  pub state: Option<u8>,
  pub state_meta: Option<f32>,
  pub area: Option<u32>,
  pub world: Option<String>,
  pub downed: Option<bool>,
}
impl PartialPlayer {
  pub fn write_package(value: &PartialPlayer, writer: &mut BufferWriter) {
    writer.write_var_u32(3);
    let mut bitmask: u32 = 0;
    if value.name.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::name as u32;
    }
    if value.x.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::x as u32;
    }
    if value.y.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::y as u32;
    }
    if value.radius.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::radius as u32;
    }
    if value.speed.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::speed as u32;
    }
    if value.energy.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::energy as u32;
    }
    if value.max_energy.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::max_energy as u32;
    }
    if value.death_timer.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::death_timer as u32;
    }
    if value.state.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::state as u32;
    }
    if value.state_meta.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::state_meta as u32;
    }
    if value.area.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::area as u32;
    }
    if value.world.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::world as u32;
    }
    if value.downed.is_some() {
      bitmask |= 1 << PartialPlayerBitmask::downed as u32;
    }
    writer.write_var_u32(bitmask);
    writer.write_var_u32(value.id as u32);
    if let Some(name) = &value.name {
      writer.write_string(name.clone());
    }
    if let Some(x) = &value.x {
      writer.write_i16(Quantizer::from_f32_to_q16(*x, 0.5));
    }
    if let Some(y) = &value.y {
      writer.write_i16(Quantizer::from_f32_to_q16(*y, 0.5));
    }
    if let Some(radius) = &value.radius {
      writer.write_i16(Quantizer::from_f32_to_q16(*radius, 0.5));
    }
    if let Some(speed) = &value.speed {
      writer.write_i16(Quantizer::from_f32_to_q16(*speed, 0.5));
    }
    if let Some(energy) = &value.energy {
      writer.write_i16(Quantizer::from_f32_to_q16(*energy, 0.5));
    }
    if let Some(max_energy) = &value.max_energy {
      writer.write_i16(Quantizer::from_f32_to_q16(*max_energy, 0.5));
    }
    if let Some(death_timer) = &value.death_timer {
      writer.write_i8(Quantizer::from_f32_to_q8(*death_timer, 0.6));
    }
    if let Some(state) = &value.state {
      writer.write_u8(*state);
    }
    if let Some(state_meta) = &value.state_meta {
      writer.write_f32(*state_meta);
    }
    if let Some(area) = &value.area {
      writer.write_var_u32(*area);
    }
    if let Some(world) = &value.world {
      writer.write_string(world.clone());
    }
    if let Some(downed) = &value.downed {
      writer.write_bool(*downed);
    }
  }
}
#[derive(Debug, Clone)]
pub struct PackedEntity {
  pub id: u64,
  pub type_id: u32,
  pub x: f32,
  pub y: f32,
  pub radius: f32,
  pub harmless: bool,
  pub state: u8,
  pub state_meta: f32,
  pub alpha: f32,
}
impl PackedEntity {
  pub fn write_package(value: &PackedEntity, writer: &mut BufferWriter) {
    writer.write_var_u32(4);
    writer.write_var_u32(value.id as u32);
    writer.write_var_u32(value.type_id);
    writer.write_f32(value.x);
    writer.write_f32(value.y);
    writer.write_f32(value.radius);
    writer.write_bool(value.harmless);
    writer.write_u8(value.state);
    writer.write_f32(value.state_meta);
    writer.write_f32(value.alpha);
  }
}
#[repr(u32)]
pub enum PartialEntityBitmask {
  x = 0,
  y = 1,
  radius = 2,
  harmless = 3,
  state = 4,
  state_meta = 5,
  alpha = 6,
}
#[derive(Debug, Clone)]
pub struct PartialEntity {
  pub id: u64,
  pub x: Option<f32>,
  pub y: Option<f32>,
  pub radius: Option<f32>,
  pub harmless: Option<bool>,
  pub state: Option<u8>,
  pub state_meta: Option<f32>,
  pub alpha: Option<f32>,
}
impl PartialEntity {
  pub fn write_package(value: &PartialEntity, writer: &mut BufferWriter) {
    writer.write_var_u32(5);
    let mut bitmask: u32 = 0;
    if value.x.is_some() {
      bitmask |= 1 << PartialEntityBitmask::x as u32;
    }
    if value.y.is_some() {
      bitmask |= 1 << PartialEntityBitmask::y as u32;
    }
    if value.radius.is_some() {
      bitmask |= 1 << PartialEntityBitmask::radius as u32;
    }
    if value.harmless.is_some() {
      bitmask |= 1 << PartialEntityBitmask::harmless as u32;
    }
    if value.state.is_some() {
      bitmask |= 1 << PartialEntityBitmask::state as u32;
    }
    if value.state_meta.is_some() {
      bitmask |= 1 << PartialEntityBitmask::state_meta as u32;
    }
    if value.alpha.is_some() {
      bitmask |= 1 << PartialEntityBitmask::alpha as u32;
    }
    writer.write_var_u32(bitmask);
    writer.write_var_u32(value.id as u32);
    if let Some(x) = &value.x {
      writer.write_i16(Quantizer::from_f32_to_q16(*x, 0.5));
    }
    if let Some(y) = &value.y {
      writer.write_i16(Quantizer::from_f32_to_q16(*y, 0.5));
    }
    if let Some(radius) = &value.radius {
      writer.write_i16(Quantizer::from_f32_to_q16(*radius, 0.5));
    }
    if let Some(harmless) = &value.harmless {
      writer.write_bool(*harmless);
    }
    if let Some(state) = &value.state {
      writer.write_u8(*state);
    }
    if let Some(state_meta) = &value.state_meta {
      writer.write_i16(Quantizer::from_f32_to_q16(*state_meta, 0.5));
    }
    if let Some(alpha) = &value.alpha {
      writer.write_i8(Quantizer::from_f32_to_q8(*alpha, 0.39));
    }
  }
}
#[derive(Debug, Clone)]
pub struct PackedArea {
  pub w: f32,
  pub h: f32,
  pub area: u32,
  pub world: String,
  pub entities: Vec<PackedEntity>,
}
impl PackedArea {
  pub fn write_package(value: &PackedArea, writer: &mut BufferWriter) {
    writer.write_var_u32(6);
    writer.write_i16(Quantizer::from_f32_to_q16(value.w, 0.5));
    writer.write_i16(Quantizer::from_f32_to_q16(value.h, 0.5));
    writer.write_u32(value.area);
    writer.write_string(value.world.clone());
    writer.write_var_u32(value.entities.len() as u32);
    for i in &value.entities {
      PackedEntity::write_package(i, writer);
    }
  }
}
#[derive(Debug, Clone)]
pub struct Players {
  pub players: Vec<PackedPlayer>,
}
impl Players {
  pub fn write_package(value: &Players, writer: &mut BufferWriter) {
    writer.write_var_u32(7);
    writer.write_var_u32(value.players.len() as u32);
    for i in &value.players {
      PackedPlayer::write_package(i, writer);
    }
  }
}
#[derive(Debug, Clone)]
pub struct Entities {
  pub entities: Vec<PackedEntity>,
}
impl Entities {
  pub fn write_package(value: &Entities, writer: &mut BufferWriter) {
    writer.write_var_u32(8);
    writer.write_var_u32(value.entities.len() as u32);
    for i in &value.entities {
      PackedEntity::write_package(i, writer);
    }
  }
}
#[derive(Debug, Clone)]
pub struct ClosePlayer {
  pub id: u32,
}
impl ClosePlayer {
  pub fn write_package(value: &ClosePlayer, writer: &mut BufferWriter) {
    writer.write_var_u32(9);
    writer.write_var_u32(value.id);
  }
}
#[derive(Debug, Clone)]
pub struct CloseEntities {
  pub ids: Vec<u32>,
}
impl CloseEntities {
  pub fn write_package(value: &CloseEntities, writer: &mut BufferWriter) {
    writer.write_var_u32(10);
    writer.write_var_u32(value.ids.len() as u32);
    for i in &value.ids {
      writer.write_var_u32(*i);
    }
  }
}
#[derive(Debug, Clone)]
pub struct UpdateEntities {
  pub items: Vec<PartialEntity>,
}
impl UpdateEntities {
  pub fn write_package(value: &UpdateEntities, writer: &mut BufferWriter) {
    writer.write_var_u32(11);
    writer.write_var_u32(value.items.len() as u32);
    for i in &value.items {
      PartialEntity::write_package(i, writer);
    }
  }
}
#[derive(Debug, Clone)]
pub struct UpdatePlayers {
  pub items: Vec<PartialPlayer>,
}
impl UpdatePlayers {
  pub fn write_package(value: &UpdatePlayers, writer: &mut BufferWriter) {
    writer.write_var_u32(12);
    writer.write_var_u32(value.items.len() as u32);
    for i in &value.items {
      PartialPlayer::write_package(i, writer);
    }
  }
}
#[repr(u32)]
pub enum PackageBitmask {
  new_player = 0,
  close_player = 1,
  players = 2,
  new_entities = 3,
  close_entities = 4,
  area_init = 5,
  myself = 6,
  update_entities = 7,
  update_players = 8,
  chat = 9,
}
#[derive(Debug, Clone)]
pub struct Package {
  pub new_player: Option<PackedPlayer>,
  pub close_player: Option<ClosePlayer>,
  pub players: Option<Players>,
  pub new_entities: Option<Entities>,
  pub close_entities: Option<CloseEntities>,
  pub area_init: Option<PackedArea>,
  pub myself: Option<PackedPlayer>,
  pub update_entities: Option<UpdateEntities>,
  pub update_players: Option<UpdatePlayers>,
  pub chat: Option<Chat>,
}
impl Package {
  pub fn write_package(value: &Package, writer: &mut BufferWriter) {
    writer.write_var_u32(13);
    let mut bitmask: u32 = 0;
    if value.new_player.is_some() {
      bitmask |= 1 << PackageBitmask::new_player as u32;
    }
    if value.close_player.is_some() {
      bitmask |= 1 << PackageBitmask::close_player as u32;
    }
    if value.players.is_some() {
      bitmask |= 1 << PackageBitmask::players as u32;
    }
    if value.new_entities.is_some() {
      bitmask |= 1 << PackageBitmask::new_entities as u32;
    }
    if value.close_entities.is_some() {
      bitmask |= 1 << PackageBitmask::close_entities as u32;
    }
    if value.area_init.is_some() {
      bitmask |= 1 << PackageBitmask::area_init as u32;
    }
    if value.myself.is_some() {
      bitmask |= 1 << PackageBitmask::myself as u32;
    }
    if value.update_entities.is_some() {
      bitmask |= 1 << PackageBitmask::update_entities as u32;
    }
    if value.update_players.is_some() {
      bitmask |= 1 << PackageBitmask::update_players as u32;
    }
    if value.chat.is_some() {
      bitmask |= 1 << PackageBitmask::chat as u32;
    }
    writer.write_var_u32(bitmask);
    if let Some(new_player) = &value.new_player {
      PackedPlayer::write_package(new_player, writer);
    }
    if let Some(close_player) = &value.close_player {
      ClosePlayer::write_package(close_player, writer);
    }
    if let Some(players) = &value.players {
      Players::write_package(players, writer);
    }
    if let Some(new_entities) = &value.new_entities {
      Entities::write_package(new_entities, writer);
    }
    if let Some(close_entities) = &value.close_entities {
      CloseEntities::write_package(close_entities, writer);
    }
    if let Some(area_init) = &value.area_init {
      PackedArea::write_package(area_init, writer);
    }
    if let Some(myself) = &value.myself {
      PackedPlayer::write_package(myself, writer);
    }
    if let Some(update_entities) = &value.update_entities {
      UpdateEntities::write_package(update_entities, writer);
    }
    if let Some(update_players) = &value.update_players {
      UpdatePlayers::write_package(update_players, writer);
    }
    if let Some(chat) = &value.chat {
      Chat::write_package(chat, writer);
    }
  }
}
#[derive(Debug, Clone)]
pub struct Packages {
  pub items: Vec<Package>,
}
impl Packages {
  pub fn write_package(value: &Packages, writer: &mut BufferWriter) {
    writer.write_var_u32(14);
    writer.write_var_u32(value.items.len() as u32);
    for i in &value.items {
      Package::write_package(i, writer);
    }
  }
}
