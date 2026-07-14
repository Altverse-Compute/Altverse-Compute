use crate::resources::assets::heroes::Hero;
use crate::resources::assets::heroes::maven::Maven;
use crate::resources::player::{Player, PlayerField};
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{Boundary, PlayerUpdateProps};
use napi::{Error, Status};

macro_rules! hero_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      HeroWrapper::Maven(v) => v.$method($($arg),*),
    }
  };
}

#[derive(Clone)]
pub enum HeroWrapper {
  Maven(Maven),
}

impl HeroWrapper {
  pub fn new(name: &str, props: JoinProps) -> Result<Self, Error> {
    match name {
      "maven" => Ok(HeroWrapper::Maven(Maven::new(props))),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown hero type: ".to_string() + name,
      )),
    }
  }
  pub fn update(&mut self, props: &mut PlayerUpdateProps) {
    hero_dispatch!(self, update(props));
  }
  pub fn input(&mut self, input: &mut Input) {
    hero_dispatch!(self, input(input));
  }
  pub fn knock(&mut self) {
    hero_dispatch!(self, knock());
  }
  pub fn res(&mut self) {
    hero_dispatch!(self, res());
  }
  pub fn collide(&mut self, boundary: Boundary) {
    hero_dispatch!(self, collide(boundary));
  }

  pub fn get_changes(&self) -> u32 {
    hero_dispatch!(self, get_changes())
  }
  pub fn clear_changes(&mut self) {
    hero_dispatch!(self, clear_changes());
  }

  pub fn player(&self) -> &Player {
    hero_dispatch!(self, player())
  }

  pub fn player_mut(&mut self) -> &mut Player {
    hero_dispatch!(self, player_mut())
  }
}
