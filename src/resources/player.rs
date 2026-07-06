use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use crate::resources::utils::vector::Vector;
use crate::resources::{distance, Boundary, PlayerUpdateProps};
use crate::CONFIG;
use track_changes_derive::TrackChanges;

#[derive(Clone, Debug, TrackChanges)]
pub struct Player {
  pub name: String,
  pub id: u64,
  pub(crate) pos: Vector,
  pub(crate) radius: f32,
  pub vel: Vector,
  acc: Vector,
  slide: Vector,
  pub speed: f32,
  pub energy: f32,
  pub max_energy: f32,
  pub downed: bool,
  pub regeneration: f32,
  pub world: String,
  pub area: u64,
  #[track(skip)]
  angle: f32,
  pub death_timer: f32,

  pub immortal: bool,
  pub state: u8,
  pub state_meta: f32,
  #[track(skip)]
  pub to_delete: bool,
  pub hero: u32,

  pub changes: Vec<PlayerField>,
}

impl Player {
  pub fn new(props: JoinProps) -> Self {
    let spawn = CONFIG.lock().unwrap().clone().spawn;
    Player {
      name: props.name,
      id: props.id,
      pos: Vector::rand(spawn.sx, spawn.sy, spawn.ex, spawn.ey),
      radius: spawn.radius,
      vel: Vector::new(None, None),
      acc: Vector::new(None, None),
      slide: Vector::new(None, None),
      speed: spawn.speed,
      energy: spawn.energy,
      max_energy: spawn.max_energy,
      downed: false,
      regeneration: spawn.regeneration,
      angle: 0.0,
      death_timer: spawn.died_timer,
      immortal: false,
      state: 0,
      state_meta: 0.0,
      world: spawn.world,
      area: spawn.area as u64,
      to_delete: false,
      hero: 0,
      changes: Vec::new(),
    }
  }

  pub fn update(&mut self, props: &PlayerUpdateProps) {
    let time_fix = props.time_fix;

    let mut slide = [self.slide.x, self.slide.y];

    let dim = 1.0 - 0.75;

    slide[0] *= 1.0 - (1.0 - dim) * time_fix;
    slide[1] *= 1.0 - (1.0 - dim) * time_fix;

    self.acc.x *= time_fix;
    self.acc.y *= time_fix;

    self.acc.x += slide[0];
    self.acc.y += slide[1];

    self.vel.x = self.acc.x;
    self.vel.y = self.acc.y;

    if self.downed {
      self.vel.x = 0.0;
      self.vel.y = 0.0;
    }

    self.pos.x += self.vel.x * time_fix;
    self.pos.y += self.vel.y * time_fix;

    self.pos.x = (self.pos.x * 100.0).round() / 100.0;
    self.pos.y = (self.pos.y * 100.0).round() / 100.0;

    self.slide.x = self.acc.x;
    self.slide.y = self.acc.y;
    self.acc = Vector::new(None, None);

    self.regenerate_energy(props.delta);
    if self.downed {
      self.death_timer -= props.delta as f32 / 1000.0;
      if self.death_timer < 0.0 {
        self.to_delete = true;
      }
    }

    self.changed_pos();
    self.changed_death_timer();

    //
    // for effect in self.effects.clone().values_mut() {
    //   effect.disable(self);
    // }
    // self.effects.clear();
  }

  fn regenerate_energy(&mut self, delta: i64) {
    self.energy += self.regeneration * (delta as f32 / 1000.0);
    if self.energy > self.max_energy {
      self.energy = self.max_energy;
    }
    self.changed_energy();
  }

  pub fn input(&mut self, input: &mut Input) {
    let shift: f32 = if input.shift { 0.5 } else { 1.0 };

    if input.left {
      self.acc.x = -self.speed * shift;
    }
    if input.right {
      self.acc.x = self.speed * shift;
    }
    if input.up {
      self.acc.y = -self.speed * shift;
    }
    if input.down {
      self.acc.y = self.speed * shift;
    }
    if input.mouse_enable {
      let dist = distance(input.mouse_pos_x, input.mouse_pos_y);
      let mut speed_x = input.mouse_pos_x;
      let mut speed_y = input.mouse_pos_y;

      if dist > 150.0 {
        speed_x = input.mouse_pos_x * (150.0 / dist);
        speed_y = input.mouse_pos_y * (150.0 / dist);
      }

      self.angle = speed_y.atan2(speed_x);

      let mouse_dist = (input.mouse_pos_x.powf(2.0) + input.mouse_pos_y.powf(2.0))
        .sqrt()
        .min(150.0);

      let mut dist_movement = self.speed * shift;
      dist_movement *= mouse_dist / 150.0;

      self.acc.x = dist_movement * self.angle.cos();
      self.acc.y = dist_movement * self.angle.sin();
    }
  }

  pub fn knock(&mut self) {
    self.downed = true;
    self.death_timer = 60.0;
    self.changed_downed();
    self.changed_death_timer();
  }

  pub fn res(&mut self) {
    self.downed = false;
    self.changed_downed();
  }

  pub fn collide(&mut self, boundary: Boundary) {
    if self.pos.x - self.radius < boundary.x {
      self.pos.x = boundary.x + self.radius;
      self.changed_pos();
    }
    if self.pos.x + self.radius > boundary.x + boundary.w {
      self.pos.x = boundary.x + boundary.w - self.radius;
      self.changed_pos();
    }
    if self.pos.y - self.radius < boundary.y {
      self.pos.y = boundary.y + self.radius;
      self.changed_pos();
    }
    if self.pos.y + self.radius > boundary.y + boundary.h {
      self.pos.y = boundary.y + boundary.h - self.radius;
      self.changed_pos();
    }
  }

  pub fn get_changes(&self) -> Vec<PlayerField> {
    self.changes.clone()
  }

  pub fn clear_changes(&mut self) {
    self.changes = Vec::new();
  }

  // pub fn pack(&self) -> PackedPlayer {
  //   PackedPlayer {
  //     id: self.id as u32,
  //     name: self.name.clone(),
  //     x: (self.pos.x * 2.0).round() as i32,
  //     y: (self.pos.y * 2.0).round() as i32,
  //     radius: (self.radius * 2.0).round().abs() as u32,
  //     speed: (self.speed * 2.0).round().abs() as u32,
  //     energy: (self.energy * 2.0).round().abs() as u32,
  //     max_energy: (self.max_energy * 2.0).round().abs() as u32,
  //     death_timer: self.death_timer.round() as u32,
  //     state: self.state as u32,
  //     area: self.area as u32,
  //     world: self.world.clone(),
  //     died: self.downed,
  //     state_meta: (self.state_meta * 2.0).round().abs() as u32,
  //     hero: self.hero,
  //   }
  // }
}
