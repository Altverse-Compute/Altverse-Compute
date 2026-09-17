use crate::resources::assets::heroes::Hero;
use crate::resources::assets::heroes::ids::MAGMAX_ID;
use crate::resources::entity::Entity;
use crate::resources::player::Player;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{Boundary, PlayerUpdateProps};

#[derive(Clone)]
pub struct Magmax {
  player: Player,
  harden: bool,
  flow: bool,
}

impl Magmax {
  pub fn new(props: JoinProps) -> Self {
    let mut player = Player::new(props);
    player.hero = MAGMAX_ID;
    Self {
      player,
      harden: false,
      flow: false,
    }
  }

  fn disable_flow(&mut self) {
    self.player.speed = self.player.reserved_speed;
    self.player.state = 0;
    self.player.changed_state();
    self.flow = false;
  }

  fn disable_harden(&mut self) {
    self.player.speed = self.player.reserved_speed;
    self.player.state = 0;
    self.player.changed_state();
    self.harden = false;
  }
}

impl Hero for Magmax {
  fn update(&mut self, props: &mut PlayerUpdateProps) {
    if self.harden {
      self.player.slide.x = 0f32;
      self.player.slide.y = 0f32;
    }

    self.player.update(props);

    if self.harden {
      self.player.energy -= 12f32 * props.delta / 1000f32;
      self.player.changed_energy();
      self.flow = false;
    }
    if self.flow {
      self.player.energy -= 1f32 * props.delta / 1000f32;
      self.player.changed_energy();
    }

    if self.player.energy < 0f32 {
      self.player.energy = 0f32;
      self.player.changed_energy();
      self.disable_flow();
      self.disable_harden();
    }
  }

  fn input(&mut self, input: &mut Input) {
    self.player.input(input);
    if input.first_ability {
      self.flow = !self.flow;
      if self.flow {
        self.player.speed = self.player.reserved_speed + 5f32;
        self.player.state = 1;
        self.player.changed_state();
      } else {
        self.disable_flow();
      }
    }
    if input.second_ability {
      self.harden = !self.harden;
      if self.harden {
        self.player.speed = 0f32;
        self.player.state = 2;
        self.player.changed_state();
      } else {
        self.disable_harden();
      }
    }
  }

  fn knock(&mut self, _: &mut Entity) {
    if !self.harden {
      self.disable_flow();
      self.disable_harden();
      self.player.knock();
    }
  }

  fn res(&mut self) {
    self.player.res();
  }

  fn collide(&mut self, boundary: Boundary) {
    self.player.collide(boundary);
  }

  fn get_changes(&self) -> u32 {
    self.player.get_changes()
  }

  fn clear_changes(&mut self) {
    self.player.clear_changes();
  }

  fn player(&self) -> &Player {
    &self.player
  }

  fn player_mut(&mut self) -> &mut Player {
    &mut self.player
  }
}
