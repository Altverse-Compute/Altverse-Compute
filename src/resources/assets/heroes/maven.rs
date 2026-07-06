use crate::resources::assets::heroes::Hero;
use crate::resources::player::{Player, PlayerField};
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{distance, Boundary, PlayerUpdateProps};

#[derive(Clone)]
pub struct Maven {
  player: Player,
  first_ability_active: bool,
  first_ability_cooldown: f64,
  second_ability_cooldown: f64,
}

impl Maven {
  pub fn new(props: JoinProps) -> Self {
    Self {
      player: Player::new(props),
      first_ability_active: false,
      first_ability_cooldown: 0.0,
      second_ability_cooldown: 0.0,
    }
  }

  fn activate_first_ability(&mut self) {
    if self.first_ability_active {
      self.first_ability_active = false;
      self.player.state = 0;
    }
    if self.player.energy > 30.0 && !self.player.downed && self.first_ability_cooldown <= 0.0 {
      self.first_ability_active = !self.first_ability_active;
      if self.first_ability_active {
        self.player.energy -= 30.0;
        self.first_ability_cooldown = 8000.0;
        self.player.state = 1;
        self.player.state_meta = 120.0;
      }
    }
  }
}

impl Hero for Maven {
  fn update(&mut self, props: &mut PlayerUpdateProps) {
    self.player.update(props);

    if self.first_ability_cooldown >= 0.0 {
      self.first_ability_cooldown -= props.delta as f64;
    }

    if self.first_ability_active {
      self.player.energy -= (props.delta as f64 / 1000.0) * 24.0;
      if self.player.energy <= 0.0 {
        self.first_ability_active = false;
        self.player.energy = 0.0;
        self.player.state = 0;
        return;
      }
      if self.player.downed {
        self.first_ability_active = false;
        self.player.state = 0;
        return;
      }

      for player in props.players.iter() {
        if distance(
          player.pos.x - self.player.pos.x,
          player.pos.y - self.player.pos.y,
        ) <= 120.0 + player.radius
          && player.downed
        {
          props
            .event_bus
            .respawn_player_and_move(player.id, self.player.pos.clone());
        }
      }
    }
  }

  fn input(&mut self, input: &mut Input) {
    self.player.input(input);
    if input.first_ability {
      self.activate_first_ability();
      input.first_ability = false;
    }
    input.second_ability = false;
  }

  fn knock(&mut self) {
    self.player.knock();
  }

  fn res(&mut self) {
    self.player.res();
  }

  fn collide(&mut self, boundary: Boundary) {
    self.player.collide(boundary);
  }

  fn get_changes(&self) -> Vec<PlayerField> {
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
