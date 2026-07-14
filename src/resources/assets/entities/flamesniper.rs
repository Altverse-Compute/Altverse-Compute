use crate::resources::assets::entities::flame::FlameTrail;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::player::Player;
use crate::resources::{distance, random, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct FlameSniper {
  entity: Entity,
  timer: f32,
}
impl FlameSniper {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.type_id = 20;
    Self {
      entity,
      timer: random(3000.0, 6000.0),
    }
  }
}

impl EntityLogic for FlameSniper {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta;

    if self.timer > 6000.0 {
      let mut target: Option<&&Player> = None;
      let mut last_distance = 20.0 * 32.0;
      for player in props.players.iter() {
        if player.pos.x > -player.radius
          && player.pos.x - player.radius < self.entity.boundary.w
          && !player.downed
        {
          let dist = distance(
            player.pos.x - self.entity.pos.x,
            player.pos.y - self.entity.pos.y,
          );
          if dist <= 20.0 * 32.0 && dist < last_distance {
            last_distance = dist;
            target = Some(player);
          }
        }

        if let Some(target) = target {
          let angl = (target.pos.y - self.entity.pos.y).atan2(target.pos.x - self.entity.pos.x);

          let mut bullet = FlameBullet::new(
            EntityProps {
              id: 1,
              type_id: 20,
              radius: self.entity.radius / 2.0,
              speed: 10.0,
              boundary: self.entity.boundary,
            },
            AdditionalEntityProps {
              count: 0,
              num: 0,
              inverse: false,
            },
          );
          bullet.entity.vel.x = angl.cos() * 10.0;
          bullet.entity.vel.y = angl.sin() * 10.0;
          bullet.entity.pos.x = self.entity.pos.x;
          bullet.entity.pos.y = self.entity.pos.y;

          props
            .event_bus
            .add_entity(EntityWrapper::FlameBullet(bullet));
          self.timer = 0.0;
        }
      }
    }
  }

  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
  }

  fn get_changes(&self) -> u8 {
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

#[derive(Clone)]
pub struct FlameBullet {
  pub entity: Entity,
  timer: f32,
}

impl FlameBullet {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    Self {
      entity: Entity::new(props.clone()),
      timer: 0.0,
    }
  }

  fn collide(entity: &mut Entity) {
    if entity.pos.x - entity.radius < entity.boundary.x {
      entity.pos.x = entity.boundary.x + entity.radius;
      entity.vel.x = entity.vel.x.abs();
      entity.to_remove = true;
    }
    if entity.pos.x + entity.radius > entity.boundary.x + entity.boundary.w {
      entity.pos.x = entity.boundary.x + entity.boundary.w - entity.radius;
      entity.vel.x = -(entity.vel.x.abs());
      entity.to_remove = true;
    }
    if entity.pos.y - entity.radius < entity.boundary.y {
      entity.pos.y = entity.boundary.y + entity.radius;
      entity.vel.y = entity.vel.y.abs();
      entity.to_remove = true;
    }
    if entity.pos.y + entity.radius > entity.boundary.y + entity.boundary.h {
      entity.pos.y = entity.boundary.y + entity.boundary.h - entity.radius;
      entity.vel.y = -(entity.vel.y.abs());
      entity.to_remove = true;
    }
  }
}

impl EntityLogic for FlameBullet {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    FlameBullet::collide(&mut self.entity);

    self.timer += props.delta;
    if self.timer >= 32.0 * ((self.entity.radius * 2.0) / self.entity.speed) {
      let mut trail = FlameTrail::new(
        EntityProps {
          id: 1,
          type_id: 19,
          radius: self.entity.radius,
          speed: 0.0,
          boundary: self.entity.boundary,
        },
        AdditionalEntityProps {
          count: 0,
          num: 0,
          inverse: false,
        },
      );
      trail.entity.pos = self.entity.pos.clone();

      trail.owner_speed = self.entity.speed;
      self.timer = 0.0;
      props.event_bus.add_entity(EntityWrapper::FlameTrail(trail));
    }
  }
  fn interact(&mut self, player: &mut HeroWrapper) {
    self.entity.interact(player);
  }

  fn get_changes(&self) -> u8 {
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
