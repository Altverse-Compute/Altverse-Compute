use std::f32::consts::PI;

use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::bubblefoam::BubbleFoam;
use crate::resources::assets::entities::magneticsoul::MagneticSoul;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::heroes::Hero;
use crate::resources::assets::heroes::ids::{BUBBLE_ID, MAVEN_ID};
use crate::resources::player::Player;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{AdditionalEntityProps, Boundary, EntityProps, PlayerUpdateProps, distance};

#[derive(Clone)]
pub struct Bubble {
  player: Player,
  bubble_foam_active: bool,
  bubble_foam_cooldown: f32,
}

const BUBBLE_FOAM_COUNT: usize = 5;
const BUBBLE_FOAM_COOLDOWN: f32 = 10000f32;

impl Bubble {
  pub fn new(props: JoinProps) -> Self {
    let mut player = Player::new(props);
    player.hero = BUBBLE_ID;
    Self {
      player,
      bubble_foam_active: false,
      bubble_foam_cooldown: 0f32,
    }
  }

  fn create_bubble_foam_at(&mut self, x: f32, y: f32, boundary: Boundary) -> BubbleFoam {
    let mut foam = BubbleFoam::new(
      EntityProps {
        id: 0,
        type_id: 0,
        radius: 20f32,
        speed: 20f32,
        boundary,
        world: self.player.world.clone(),
        area: self.player.area,
      },
      AdditionalEntityProps {
        count: 0,
        num: 1,
        inverse: false,
      },
    );
    let entity = foam.entity_mut();
    entity.pos.x = self.player.pos.x + x * self.player.radius;
    entity.pos.y = self.player.pos.y + y * self.player.radius;
    entity.changed_pos();
    entity.vel.x = x * entity.speed;
    entity.vel.y = y * entity.speed;

    foam
  }
}

impl Hero for Bubble {
  fn update(&mut self, props: &mut PlayerUpdateProps) {
    self.player.update(props);

    if self.bubble_foam_cooldown > 0.0 {
      self.bubble_foam_cooldown -= props.delta;
      self.bubble_foam_active = false;
    } else {
      self.bubble_foam_cooldown = 0.0;
    }

    if self.bubble_foam_active && self.bubble_foam_cooldown == 0f32 && self.player.energy >= 25f32 {
      self.player.energy -= 25f32;
      for i in 0..BUBBLE_FOAM_COUNT {
        let angle = -PI / 2.0 + (i as f32) * (2.0 * PI / BUBBLE_FOAM_COUNT as f32);
        let dir_x = angle.cos();
        let dir_y = angle.sin();

        props.event_bus.add_entity(
          EntityWrapper::BubbleFoam(self.create_bubble_foam_at(
            dir_x,
            dir_y,
            props.entity_boundary,
          )),
          self.player.area,
          self.player.world.clone(),
        );
      }

      self.bubble_foam_cooldown = BUBBLE_FOAM_COOLDOWN;
      self.bubble_foam_active = false;
    }

    /*if self.lifebuoy_cooldown >= 0.0 {
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
        ) <= MAVEN_LIFEBUOY_RADIUS + player.radius
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
      soul.radius_of_action = MAVEN_MAGNETIC_SOUL_RADIUS;

      props
        .event_bus
        .add_entity(EntityWrapper::MagneticSoul(soul));
      self.magnetic_soul_spawned = true;
    }

    if (self.magnetic_soul_active && !self.player.downed) || self.magnetic_soul_cooldown == 0f32 {
      self.deactivate_magnetic_soul();
    }*/
  }

  fn input(&mut self, input: &mut Input) {
    self.player.input(input);
    if input.first_ability {
      self.bubble_foam_active = true;
    }
    if input.second_ability {}
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
