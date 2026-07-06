use crate::resources::assets::entities::bee::Bee;
use crate::resources::assets::entities::cloud::Cloud;
use crate::resources::assets::entities::draining::Draining;
use crate::resources::assets::entities::drop::Drop;
use crate::resources::assets::entities::fade::Fade;
use crate::resources::assets::entities::flame::{Flame, FlameTrail};
use crate::resources::assets::entities::flamesniper::{FlameBullet, FlameSniper};
use crate::resources::assets::entities::homing::Homing;
use crate::resources::assets::entities::homingsniper::{HomingBullet, HomingSniper};
use crate::resources::assets::entities::icicle::Icicle;
use crate::resources::assets::entities::immune::Immune;
use crate::resources::assets::entities::leaf::Leaf;
use crate::resources::assets::entities::normal::Normal;
use crate::resources::assets::entities::sizer::Sizer;
use crate::resources::assets::entities::slow::Slow;
use crate::resources::assets::entities::sniper::{Bullet, Sniper};
use crate::resources::assets::entities::stormcloud::StormCloud;
use crate::resources::assets::entities::wall::Wall;
use crate::resources::assets::entities::EntityLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::{Entity, EntityField};
use crate::resources::{AdditionalEntityProps, EntityProps, EntityUpdateProps};
use napi::{Error, Status};

macro_rules! entity_dispatch {
  ($self:expr, $method:ident($($arg:expr),*)) => {
    match $self {
      EntityWrapper::Normal(v) => v.$method($($arg),*),
      EntityWrapper::Flame(v) => v.$method($($arg),*),
      EntityWrapper::FlameTrail(v) => v.$method($($arg),*),
      EntityWrapper::Fade(v) => v.$method($($arg),*),
      EntityWrapper::Wall(v) => v.$method($($arg),*),
      EntityWrapper::FlameBullet(v) => v.$method($($arg),*),
      EntityWrapper::FlameSniper(v) => v.$method($($arg),*),
      EntityWrapper::Immune(v) => v.$method($($arg),*),
      EntityWrapper::Drop(v) => v.$method($($arg),*),
      EntityWrapper::Sniper(v) => v.$method($($arg),*),
      EntityWrapper::Bullet(v) => v.$method($($arg),*),
      EntityWrapper::Bee(v) => v.$method($($arg),*),
      EntityWrapper::Homing(v) => v.$method($($arg),*),
      EntityWrapper::HomingSniper(v) => v.$method($($arg),*),
      EntityWrapper::HomingBullet(v) => v.$method($($arg),*),
      EntityWrapper::Slow(v) => v.$method($($arg),*),
      EntityWrapper::Sizer(v) => v.$method($($arg),*),
      EntityWrapper::Icicle(v) => v.$method($($arg),*),
      EntityWrapper::Draining(v) => v.$method($($arg),*),
      EntityWrapper::Leaf(v) => v.$method($($arg),*),
      EntityWrapper::Cloud(v) => v.$method($($arg),*),
      EntityWrapper::StormCloud(v) => v.$method($($arg),*),
    }
  };
}

#[derive(Clone)]
pub enum EntityWrapper {
  Normal(Normal),
  Flame(Flame),
  FlameTrail(FlameTrail),
  Fade(Fade),
  Wall(Wall),
  FlameBullet(FlameBullet),
  FlameSniper(FlameSniper),
  Immune(Immune),
  Drop(Drop),
  Sniper(Sniper),
  Bullet(Bullet),
  Bee(Bee),
  Homing(Homing),
  HomingSniper(HomingSniper),
  HomingBullet(HomingBullet),
  Slow(Slow),
  Sizer(Sizer),
  Icicle(Icicle),
  Draining(Draining),
  Leaf(Leaf),
  Cloud(Cloud),
  StormCloud(StormCloud),
}

impl EntityWrapper {
  pub fn new(
    name: &str,
    props: &mut EntityProps,
    additional: AdditionalEntityProps,
  ) -> Result<Self, Error> {
    match name {
      "normal" => Ok(EntityWrapper::Normal(Normal::new(*props, additional))),
      "flame" => Ok(EntityWrapper::Flame(Flame::new(*props, additional))),
      "fade" => Ok(EntityWrapper::Fade(Fade::new(*props, additional))),
      "wall" => Ok(EntityWrapper::Wall(Wall::new(*props, additional))),
      "immune" => Ok(EntityWrapper::Immune(Immune::new(*props, additional))),
      "flame_sniper" => Ok(EntityWrapper::FlameSniper(FlameSniper::new(
        *props, additional,
      ))),
      "drop" => Ok(EntityWrapper::Drop(Drop::new(*props, additional))),
      "homing" => Ok(EntityWrapper::Homing(Homing::new(*props, additional))),
      "bee" => Ok(EntityWrapper::Bee(Bee::new(*props, additional))),
      "sniper" => Ok(EntityWrapper::Sniper(Sniper::new(*props, additional))),
      "homing_sniper" => Ok(EntityWrapper::HomingSniper(HomingSniper::new(
        *props, additional,
      ))),
      "slower" => Ok(EntityWrapper::Slow(Slow::new(*props, additional))),
      "sizer" => Ok(EntityWrapper::Sizer(Sizer::new(*props, additional))),
      "icicle" => Ok(EntityWrapper::Icicle(Icicle::new(*props, additional))),
      "draining" => Ok(EntityWrapper::Draining(Draining::new(*props, additional))),
      "leaf" => Ok(EntityWrapper::Leaf(Leaf::new(*props, additional))),
      "cloud" => Ok(EntityWrapper::Cloud(Cloud::new(*props, additional))),
      "storm_cloud" => Ok(EntityWrapper::StormCloud(StormCloud::new(
        *props, additional,
      ))),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Unknown enemy type: ".to_string() + name,
      )),
    }
  }

  pub fn update(&mut self, props: &mut EntityUpdateProps) {
    entity_dispatch!(self, update(props));
  }

  pub fn interact(&mut self, player: &mut HeroWrapper) {
    entity_dispatch!(self, interact(player));
  }

  pub fn get_changes(&self) -> Vec<EntityField> {
    entity_dispatch!(self, get_changes())
  }
  pub fn clear_changes(&mut self) {
    entity_dispatch!(self, clear_changes());
  }

  pub fn entity(&self) -> &Entity {
    entity_dispatch!(self, entity())
  }

  pub fn entity_mut(&mut self) -> &mut Entity {
    entity_dispatch!(self, entity_mut())
  }
}
