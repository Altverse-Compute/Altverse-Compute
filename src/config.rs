use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
  pub spawn: Spawn,
  pub worlds: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spawn {
  pub radius: f32,
  pub speed: f32,
  pub max_speed: f32,
  pub regeneration: f32,
  pub energy: f32,
  pub max_energy: f32,
  pub world: String,
  pub area: u32,

  pub sx: f32,
  pub sy: f32,
  pub ex: f32,
  pub ey: f32,

  pub died_timer: f32,
}

impl Config {
  pub fn new() -> Self {
    Self {
      spawn: Spawn {
        radius: 15.0,
        speed: 17.0,
        max_speed: 17.0,
        regeneration: 7.0,
        energy: 30.0,
        max_energy: 30.0,
        world: "".to_string(),
        area: 0,
        sx: -(10.0 * 32.0 - 155.0),
        sy: 25.0 * 32.0 + 15.0,
        ex: -15.0,
        ey: 15.0 * 32.0 - 15.0 - 2.0 * 32.0,
        died_timer: 60.0,
      },
      worlds: Vec::new(),
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawWorld {
  pub name: String,
  pub areas: Vec<RawArea>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawArea {
  pub enemies: Vec<RawEntity>,
  pub w: f32,
  pub h: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawEntity {
  pub types: Vec<String>,
  pub radius: f32,
  pub speed: f32,
  pub count: u32,
}
