use std::collections::HashMap;

#[derive(Clone)]
pub enum Role {
  User = 0,
  Mod,
  Dev,
}

#[derive(Clone)]
pub struct Chat {
  pub id: u64,
  pub content: String,
  pub author: String,
  pub role: Role,
  pub world: String,
}

#[derive(Clone)]
pub struct PackedArea {
  pub w: f32,
  pub h: f32,
  pub area: u32,
  pub world: String,
  pub entities: Vec<u64>,
}

#[derive(Clone)]
pub enum Package {
  NewPlayer(u64),
  ClosePlayer(u64),
  Players(HashMap<u32, u64>),
  UpdatePlayers(Vec<u64>),
  Myself(i64),

  NewEntities(Vec<u64>),
  UpdateEntities(Vec<u64>),
  CloseEntities(Vec<u64>),

  AreaInit(PackedArea),
  Chat(Chat),
}
