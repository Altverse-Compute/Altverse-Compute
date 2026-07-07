use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::player::Player;
use crate::resources::{distance, AdditionalEntityProps, EntityProps, EntityUpdateProps};

const MAX_DIST: f32 = 5.625 * 32.0;
const ANGLE_INCREMENT: f32 = 0.04;

#[derive(Clone)]
pub struct Homing {
  entity: Entity,
}

impl Homing {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 9;
    Self { entity }
  }
}

impl EntityLogic for Homing {
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
    self.entity.collide();
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
