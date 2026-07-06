use crate::resources::assets::hero::HeroWrapper;
use crate::resources::utils::vector::Vector;
use crate::resources::{distance, random, Boundary, EntityProps, EntityUpdateProps};
use std::f32::consts::PI;
use track_changes_derive::TrackChanges;

#[derive(Clone, Debug, TrackChanges)]
pub struct Entity {
  #[track(skip)]
  pub id: u64,
  pub type_id: u64,
  pub radius: f32,
  pub speed: f32,
  pub harmless: bool,
  #[track(skip)]
  pub immune: bool,
  #[track(skip)]
  pub angle: f32,
  pub pos: Vector,
  #[track(skip)]
  pub vel: Vector,
  #[track(skip)]
  pub to_remove: bool,
  #[track(skip)]
  pub friction: f32,
  #[track(skip)]
  pub aura: f32,
  #[track(skip)]
  pub boundary: Boundary,

  pub state: u8,
  pub state_metadata: f32,
  pub alpha: f32,

  pub changes: Vec<EntityField>,
}

impl Entity {
  pub fn new(props: EntityProps) -> Self {
    let angle = random(0.0, 1.0);
    Self {
      id: props.id,
      type_id: props.type_id,
      radius: props.radius,
      speed: props.speed,
      immune: false,
      angle,
      pos: Vector::rand(
        props.boundary.x,
        props.boundary.y,
        props.boundary.x + props.boundary.w,
        props.boundary.y + props.boundary.h,
      ),
      vel: Vector::from_angle(angle * PI * 2.0, props.speed),
      harmless: false,
      to_remove: false,
      friction: 0.0,
      boundary: props.boundary,

      state: 0,
      state_metadata: 0.0,
      alpha: 1.0,
      aura: 0.0,

      changes: Vec::new(),
    }
  }

  pub fn update(&mut self, props: &EntityUpdateProps) {
    self.movement(props.time_fix);
  }

  pub fn movement(&mut self, time_fix: f32) {
    self.pos.x += self.vel.x * time_fix;
    self.pos.y += self.vel.y * time_fix;
    self.changed_pos();

    let dim = 1.0 - self.friction * time_fix;
    self.vel.x *= dim;
    self.vel.y *= dim;
  }

  pub fn angle_to_vel(&mut self) {
    self.vel.x = self.angle.cos() * self.speed;
    self.vel.y = self.angle.sin() * self.speed;
  }

  pub fn vel_to_angle(&mut self) {
    self.angle = self.vel.y.atan2(self.vel.x);
    let dist = distance(0.0 - self.vel.x, 0.0 - self.vel.y);
    self.speed = dist;
  }

  pub fn collide(&mut self) {
    if self.pos.x - self.radius < self.boundary.x {
      self.pos.x = self.boundary.x + self.radius;
      self.vel.x = self.vel.x.abs();
      self.changed_pos();
    }
    if self.pos.x + self.radius > self.boundary.x + self.boundary.w {
      self.pos.x = self.boundary.x + self.boundary.w - self.radius;
      self.vel.x = -self.vel.x.abs();
      self.changed_pos();
    }
    if self.pos.y - self.radius < self.boundary.y {
      self.pos.y = self.boundary.y + self.radius;
      self.vel.y = self.vel.y.abs();
      self.changed_pos();
    }
    if self.pos.y + self.radius > self.boundary.y + self.boundary.h {
      self.pos.y = self.boundary.y + self.boundary.h - self.radius;
      self.vel.y = -(self.vel.y.abs());
      self.changed_pos();
    }
  }

  pub fn interact(&mut self, hero: &mut HeroWrapper) {
    let player = hero.player_mut();
    if !self.harmless
      && player.pos.x > -player.radius
      && player.pos.x - player.radius < self.boundary.w
    {
      if !player.immortal && !player.downed {
        if distance(player.pos.x - self.pos.x, player.pos.y - self.pos.y)
          <= self.radius + player.radius
        {
          player.knock();
        }
      }
    }
  }

  pub fn get_changes(&self) -> Vec<EntityField> {
    self.changes.clone()
  }

  pub fn clear_changes(&mut self) {
    self.changes = Vec::new();
  }

  // pub fn pack(&self) -> PackedEntity {
  //   PackedEntity {
  //     type_id: self.type_id as u32,
  //     x: (self.pos.x * 2.0).round() as i32,
  //     y: (self.pos.y * 2.0).round() as i32,
  //     radius: (self.radius * 2.0).round().abs() as u32,
  //     harmless: self.harmless,
  //     state: self.state as u32,
  //     state_metadata: (self.state_metadata * 2.0).round().abs() as u32,
  //     alpha: (self.alpha * 20.0).round().abs() as u32,
  //   }
  // }
}
