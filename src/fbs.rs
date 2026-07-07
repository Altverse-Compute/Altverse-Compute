#[derive(Clone, Debug)]
pub enum Role {
  User = 0,
  Mod,
  Dev,
}

#[derive(Clone, Debug)]
pub struct Chat {
  pub id: u64,
  pub content: String,
  pub author: String,
  pub role: Role,
  pub world: String,
}

#[derive(Clone, Debug)]
pub struct PackedArea {
  pub w: f32,
  pub h: f32,
  pub area: u32,
  pub world: String,
  pub entities: Vec<u64>,
}

#[derive(Clone, Debug)]
pub enum Package {
  NewPlayer(u64),
  ClosePlayer(u64),
  Players(Vec<u64>),
  UpdatePlayers(Vec<u64>),
  Myself(u64),

  NewEntities((Vec<u64>, String, usize)),
  UpdateEntities((Vec<u64>, String, usize)),
  CloseEntities(Vec<u64>),

  AreaInit(PackedArea),
  Chat(Chat),
}
