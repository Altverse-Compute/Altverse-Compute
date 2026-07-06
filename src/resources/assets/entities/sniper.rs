use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::player::Player;
use crate::resources::{distance, random, AdditionalEntityProps, EntityProps, EntityUpdateProps};

#[derive(Clone)]
pub struct Sniper {
  entity: Entity,
  timer: f64,
}

impl Sniper {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props);
    entity.type_id = 3;
    Self {
      entity,
      timer: random(0.0, 3000.0),
    }
  }
}

impl EntityLogic for Sniper {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    self.entity.collide();

    self.timer += props.delta as f64;

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

          let mut bullet = Bullet::new(
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

          props.event_bus.add_entity(EntityWrapper::Bullet(bullet));

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
pub struct Bullet {
  pub entity: Entity,
}
impl Bullet {
  pub fn new(props: EntityProps, _: AdditionalEntityProps) -> Self {
    let mut entity = Entity::new(props.clone());
    entity.type_id = 4;
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

impl EntityLogic for Bullet {
  fn update(&mut self, props: &mut EntityUpdateProps) {
    self.entity.update(props);
    Bullet::collide(&mut self.entity);
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
