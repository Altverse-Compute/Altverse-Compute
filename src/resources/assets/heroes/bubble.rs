use std::f32::consts::PI;

use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entities::bubblefoam::BubbleFoam;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::heroes::Hero;
use crate::resources::assets::heroes::ids::BUBBLE_ID;
use crate::resources::entity::Entity;
use crate::resources::player::Player;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::{AdditionalEntityProps, Boundary, EntityProps, PlayerUpdateProps};

#[derive(Clone)]
pub struct Bubble {
  player: Player,
  bubble_foam_active: bool,
  bubble_foam_cooldown: f32,
  bubble_wrap_created: bool,
  bubble_wrap_cooldown: f32,
  bubble_wrap_creating_cooldown: f32,
}

const BUBBLE_FOAM_COUNT: usize = 5;
const BUBBLE_FOAM_COOLDOWN: f32 = 10000f32;
const BUBBLE_WRAP_CREATE_COOLDOWN: f32 = 6000f32;
const BUBBLE_WRAP_COOLDOWN: f32 = 12000f32;

impl Bubble {
  pub fn new(props: JoinProps) -> Self {
    let mut player = Player::new(props);
    player.hero = BUBBLE_ID;
    Self {
      player,
      bubble_foam_active: false,
      bubble_foam_cooldown: 0f32,
      bubble_wrap_created: false,
      bubble_wrap_cooldown: 0f32,
      bubble_wrap_creating_cooldown: 0f32,
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

  fn created_bubble_wrap(&mut self) {
    self.player.state = 1;
    self.player.changed_state();
    self.player.state_meta = 16f32;
    self.player.changed_state_meta();
  }

  fn destroyed_bubble_wrap(&mut self) {
    self.player.state = 0;
    self.player.changed_state();
    self.player.state_meta = 0f32;
    self.player.changed_state_meta();
  }

  fn magnitude(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
  }

  fn unit(x: f32, y: f32) -> (f32, f32) {
    let m = Bubble::magnitude(x, y);
    if m == 0.0 { (0.0, 0.0) } else { (x / m, y / m) }
  }
}

impl Hero for Bubble {
  fn update(&mut self, props: &mut PlayerUpdateProps) {
    self.player.update(props);

    if self.bubble_foam_cooldown > 0.0 {
      self.bubble_foam_cooldown -= props.delta;
    } else {
      self.bubble_foam_cooldown = 0.0;
    }

    if self.bubble_wrap_creating_cooldown > 0.0 {
      self.bubble_wrap_creating_cooldown -= props.delta;
      if self.bubble_wrap_creating_cooldown < 100.0 {
        self.bubble_wrap_created = true;
        self.created_bubble_wrap();
      }
    }
    if self.bubble_wrap_cooldown > 0.0 {
      self.bubble_wrap_cooldown -= props.delta;
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
    }
    self.bubble_foam_active = false;
  }

  fn input(&mut self, input: &mut Input) {
    self.player.input(input);
    if input.first_ability {
      self.bubble_foam_active = true;
    }
    if input.second_ability {
      self.bubble_wrap_creating_cooldown = BUBBLE_WRAP_CREATE_COOLDOWN;
      self.bubble_wrap_cooldown = BUBBLE_WRAP_COOLDOWN;
    }
  }

  fn knock(&mut self, entity: &mut Entity) {
    if !self.bubble_wrap_created || entity.immune {
      self.player.knock();
      return;
    }

    let dist_vec_x = entity.pos.x - self.player.pos.x;
    let dist_vec_y = entity.pos.y - self.player.pos.y;
    let dist: f32 = Bubble::magnitude(dist_vec_x, dist_vec_y);

    let pen_depth = self.player.radius + entity.radius - dist;
    let (unit_x, unit_y) = Bubble::unit(dist_vec_x, dist_vec_y);
    let pen_x = unit_x * (pen_depth / 2.0);
    let pen_y = unit_y * (pen_depth / 2.0);

    entity.pos.x += pen_x;
    entity.pos.y += pen_y;
    self.player.pos.x -= pen_x;
    self.player.pos.y -= pen_y;

    entity.changed_pos();
    self.player.changed_pos();

    let angle_to_entity =
      (entity.pos.y - self.player.pos.y).atan2(entity.pos.x - self.player.pos.x);

    entity.vel.x = angle_to_entity.cos() * entity.speed;
    entity.vel.y = angle_to_entity.sin() * entity.speed;
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
