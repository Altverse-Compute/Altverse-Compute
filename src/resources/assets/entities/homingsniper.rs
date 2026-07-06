use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::player::Player;
use crate::resources::{distance, random, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct HomingSniper {
  entity: Entity,
  timer: f32,
}

const MAX_DIST: f32 = 5.625 * 32.0;
const ANGLE_INCREMENT: f32 = 0.04;

impl HomingSniper {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 16;
    Self {
      entity,
      timer: random(0.0, 3000.0),
    }
  }
}

impl EntityLogic for HomingSniper {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta;

    if self.timer > 3000.0 {
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

          let mut bullet = HomingBullet::new(
            EntityProps {
              id: 1,
              type_id: 3,
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
            .add_entity(EntityWrapper::HomingBullet(bullet));

          self.timer = 0.0;
        }
      }
    }
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

#[derive(Clone)]
pub struct HomingBullet {
  pub entity: Entity,
}
impl HomingBullet {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.type_id = 17;
    Self { entity }
  }
  fn collide(entity: &mut Entity) {
    if entity.pos.x - entity.radius < entity.boundary.x {
      entity.to_remove = true;
    }
    if entity.pos.x + entity.radius > entity.boundary.x + entity.boundary.w {
      entity.to_remove = true;
    }
    if entity.pos.y - entity.radius < entity.boundary.y {
      entity.to_remove = true;
    }
    if entity.pos.y + entity.radius > entity.boundary.y + entity.boundary.h {
      entity.to_remove = true;
    }
  }
}

impl EntityLogic for HomingBullet {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    let mut target: Option<&&Player> = None;
    let mut last_distance = MAX_DIST;
    for player in props.players.iter() {
      if player.pos.x > -player.radius
        && player.pos.x - player.radius < self.entity.boundary.w
        && !player.downed
      {
        let dist = distance(
          player.pos.x - self.entity.pos.x,
          player.pos.y - self.entity.pos.y,
        );
        if dist <= MAX_DIST && dist < last_distance {
          last_distance = dist;
          target = Some(player);
        }
      }
    }

    if let Some(target) = target {
      let angle = (target.pos.y - self.entity.pos.y).atan2(target.pos.x - self.entity.pos.x);

      let diff = angle - self.entity.angle;
      let angle_diff = diff.sin().atan2(diff.cos());

      self.entity.vel_to_angle();
      if angle_diff.abs() >= ANGLE_INCREMENT {
        if angle_diff < 0.0 {
          self.entity.angle -= ANGLE_INCREMENT * (props.delta / 30.0);
        } else {
          self.entity.angle += ANGLE_INCREMENT * (props.delta / 30.0);
        }
        self.entity.angle_to_vel();
      }
    }

    self.entity.update(props);
    HomingBullet::collide(&mut self.entity);
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
