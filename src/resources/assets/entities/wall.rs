use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::utils::vector::Vector;
use crate::resources::{AdditionalEntityProps, Boundary, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Wall {
  entity: Entity,
  dir_act: i64,
}

struct Around {
  pub pos: Vector,
  pub dir: i64,
}

impl Wall {
  pub fn new(props: EntityProps, additional: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.type_id = 1;
    let mut dir_act = 1;
    if additional.inverse {
      dir_act = -1;
    }

    entity.vel_to_angle();
    let bound = entity.boundary;
    let new_bound = Boundary {
      x: bound.x + entity.radius,
      y: bound.y + entity.radius,
      w: bound.w - entity.radius * 2.0,
      h: bound.h - entity.radius * 2.0,
    };

    entity.immune = true;

    let peri = (Wall::perimeter(new_bound) / additional.count as f64) * additional.num as f64
      + props.boundary.w / 2.0;
    let around = Wall::warp_around(&new_bound, peri);

    entity.pos = around.pos;
    if around.dir == 0 {
      entity.vel.y = 0.0;
      entity.vel.x = entity.speed * dir_act as f64;
    }
    if around.dir == 1 {
      entity.vel.x = 0.0;
      entity.vel.y = entity.speed * dir_act as f64;
    }
    if around.dir == 2 {
      entity.vel.y = 0.0;
      entity.vel.x = -entity.speed * dir_act as f64;
    }
    if around.dir == 3 {
      entity.vel.x = 0.0;
      entity.vel.y = -entity.speed * dir_act as f64;
    }

    Self { entity, dir_act }
  }

  fn perimeter(area: Boundary) -> f64 {
    return area.w * 2.0 + area.h * 2.0;
  }

  fn warp_around(bound: &Boundary, length: f64) -> Around {
    let length = length % (bound.w * 2.0 + bound.h * 2.0);
    let mut pos = Vector::new(None, None);
    let mut dir: i64 = 0;
    if length < bound.w {
      dir = 0;
      pos.y = bound.y;
      pos.x = bound.x + length;
    } else if length < bound.w + bound.h {
      dir = 1;
      pos.x = bound.x + bound.w;
      pos.y = bound.y + (length - bound.w);
    } else if length < bound.w * 2.0 + bound.h {
      dir = 2;
      pos.y = bound.y + bound.h;
      pos.x = bound.x + bound.w - (length - (bound.w + bound.h));
    } else if length < bound.w * 2.0 + bound.h * 2.0 {
      dir = 3;
      pos.x = bound.x;
      pos.y = bound.y + bound.h - (length - (bound.w * 2.0 + bound.h));
    }

    Around { pos, dir }
  }

  pub fn collide(&mut self) {
    let entity = &mut self.entity;
    if entity.pos.x - entity.radius < entity.boundary.x {
      entity.pos.x = entity.radius + entity.boundary.x + 1.0;
      entity.changed_pos();
      entity.vel.x = 0.0;
      entity.vel.y = -entity.speed * self.dir_act as f64;
    }
    if entity.pos.x + entity.radius > entity.boundary.x + entity.boundary.w {
      entity.pos.x = entity.boundary.w - entity.radius + entity.boundary.x;
      entity.changed_pos();
      entity.vel.x = 0.0;
      entity.vel.y = entity.speed * self.dir_act as f64;
    }
    if entity.pos.y - entity.radius < entity.boundary.y {
      entity.pos.y = entity.radius + entity.boundary.y + 1.0;
      entity.changed_pos();
      entity.vel.y = 0.0;
      entity.vel.x = entity.speed * self.dir_act as f64;
    }
    if entity.pos.y + entity.radius > entity.boundary.y + entity.boundary.h {
      entity.pos.y = entity.boundary.h - entity.radius + entity.boundary.y;
      entity.changed_pos();
      entity.vel.y = 0.0;
      entity.vel.x = -entity.speed * self.dir_act as f64;
    }
  }
}

impl EntityLogic for Wall {
  fn update(&mut self, props: &mut EntityUpdateProps<'_>) {
    self.entity.update(props);
    self.collide();
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
  }

  fn get_changes(&self) -> Vec<EntityField> {
    self.entity.get_changes()
  }

  fn clear_changes(&mut self) {
    self.entity.clear_changes();
  }

  fn entity(&self) -> &Entity {
    &self.entity
  }

  fn entity_mut(&mut self) -> &mut Entity {
    &mut self.entity
  }
}
