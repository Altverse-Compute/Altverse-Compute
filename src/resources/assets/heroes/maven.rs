use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::magneticsoul::MagneticSoul;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::heroes::Hero;
use crate::resources::assets::heroes::ids::MAVEN_ID;
use crate::resources::player::Player;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{AdditionalEntityProps, Boundary, EntityProps, PlayerUpdateProps, distance};

#[derive(Clone)]
pub struct Maven {
  player: Player,
  lifebuoy_active: bool,
  lifebuoy_cooldown: f32,
  magnetic_soul_active: bool,
  magnetic_soul_cooldown: f32,
  magnetic_soul_spawned: bool,
}

impl Maven {
  pub fn new(props: JoinProps) -> Self {
    let mut player = Player::new(props);
    player.hero = MAVEN_ID;
    Self {
      player,
      lifebuoy_active: false,
      lifebuoy_cooldown: 0f32,
      magnetic_soul_active: false,
      magnetic_soul_cooldown: 0f32,
      magnetic_soul_spawned: false,
    }
  }

  fn activate_lifebuoy(&mut self) {
    if self.lifebuoy_active {
      self.deactivate_lifebuoy();
    }
    if self.player.energy > 30.0 && !self.player.downed && self.lifebuoy_cooldown <= 0.0 {
      self.lifebuoy_active = !self.lifebuoy_active;
      if self.lifebuoy_active {
        self.player.energy -= 30.0;
        self.player.changed_energy();
        self.lifebuoy_cooldown = 2000.0;
        self.player.state = 1;
        self.player.state_meta = 120.0;
        self.player.changed_state();
        self.player.changed_state_meta();
      }
    }
  }

  fn activate_magnetic_soul(&mut self) {
    if self.player.downed && self.magnetic_soul_cooldown == 0.0 {
      self.magnetic_soul_active = true;
      self.magnetic_soul_cooldown = 8000.0;
      self.player.state = 2;
      self.player.changed_state();
      self.player.state_meta = 120.0;
      self.player.changed_state_meta();
    }
  }

  fn deactivate_lifebuoy(&mut self) {
    self.lifebuoy_active = false;
    self.player.state = 0;
    self.player.changed_state();
  }

  fn deactivate_magnetic_soul(&mut self) {
    self.magnetic_soul_active = false;
    self.player.state = 0;
    self.player.changed_state();
    self.player.state_meta = 0.0;
    self.player.changed_state_meta();
    self.magnetic_soul_spawned = false;
  }
}

impl Hero for Maven {
  fn update(&mut self, props: &mut PlayerUpdateProps) {
    self.player.update(props);

    if self.lifebuoy_cooldown >= 0.0 {
      self.lifebuoy_cooldown -= props.delta;
    }
    if self.magnetic_soul_cooldown > 0.0 {
      self.magnetic_soul_cooldown -= props.delta;
    } else {
      self.magnetic_soul_cooldown = 0.0;
    }

    if self.lifebuoy_active {
      self.player.energy -= (props.delta as f32 / 1000.0) * 24.0;
      if self.player.energy <= 0.0 {
        self.player.energy = 0.0;
        self.player.changed_energy();
        self.deactivate_lifebuoy();
        return;
      }

      for player in props.players.iter() {
        if distance(
          player.pos.x - self.player.pos.x,
          player.pos.y - self.player.pos.y,
        ) <= 120.0 + player.radius
          && player.downed
          && player.id != self.player.id
        {
          props
            .event_bus
            .respawn_player_and_move(player.id, self.player.pos.clone());
        }
      }
    }

    if self.magnetic_soul_active && !self.magnetic_soul_spawned {
      let mut soul = MagneticSoul::new(
        EntityProps {
          id: 2,
          type_id: 1,
          radius: 15f32,
          speed: 0f32,
          boundary: Boundary {
            x: -10000f32,
            y: -10000f32,
            w: 10000f32,
            h: 10000f32,
          },
        },
        AdditionalEntityProps {
          count: 0,
          num: 0,
          inverse: false,
        },
      );
      let entity = soul.entity_mut();

      entity.pos = self.player.pos.clone();
      soul.start_position = self.player.pos.clone();
      soul.caster_id = self.player.id;

      props
        .event_bus
        .add_entity(EntityWrapper::MagneticSoul(soul));
      self.magnetic_soul_spawned = true;
    }

    if (self.magnetic_soul_active && !self.player.downed) || self.magnetic_soul_cooldown == 0f32 {
      self.deactivate_magnetic_soul();
    }
  }

  fn input(&mut self, input: &mut Input) {
    self.player.input(input);
    if input.first_ability {
      self.activate_lifebuoy();
    }
    if input.second_ability {
      self.activate_magnetic_soul();
    }
  }

  fn knock(&mut self) {
    self.player.knock();
    self.deactivate_lifebuoy();
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
