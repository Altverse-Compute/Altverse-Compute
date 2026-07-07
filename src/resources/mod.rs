use crate::bus::EventBus;
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::entity::Entity;
use crate::resources::player::Player;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;

pub mod area;
pub mod assets;
pub mod effect;
pub mod entity;
pub mod player;
pub mod utils;
pub mod world;

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(rand::rng());
}

// Structures

#[derive(Debug, Clone, Copy)]
pub struct Boundary {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

#[derive(Clone, Copy)]
pub struct EntityProps {
  pub id: u64,
  pub type_id: u64,
  pub radius: f32,
  pub speed: f32,
  pub boundary: Boundary,
}

pub struct EntityUpdateProps<'a> {
  pub delta: f32,
  pub time_fix: f32,
  pub players: Vec<&'a Player>,
  pub event_bus: &'a mut EventBus,
}

pub struct EffectUpdateProps<'a> {
  pub delta: f32,
  pub time_fix: f32,
  pub caster: &'a EntityWrapper,
  pub target: &'a mut HeroWrapper,
  pub boundary: Boundary,
}

pub struct PartEffectUpdateProps<'a> {
  pub delta: f32,
  pub time_fix: f32,
  pub target: &'a Player,
}

pub struct UpdateProps {
  pub delta: f32,
  pub time_fix: f32,
}

pub struct PlayerUpdateProps<'a> {
  pub delta: f32,
  pub time_fix: f32,
  pub players: Vec<&'a Player>,
  pub event_bus: &'a mut EventBus,
}

pub struct EffectProps<'a> {
  pub delta: i64,
  pub time_fix: f64,
  pub target: &'a mut Player,
  pub caster: &'a mut Entity,
}

#[derive(Clone, Copy)]
pub struct AdditionalEntityProps {
  pub count: u64,
  pub num: u64,
  pub inverse: bool,
}

// functions

pub fn distance(a: f32, b: f32) -> f32 {
  (a * a + b * b).sqrt()
}

pub fn random(min: f32, max: f32) -> f32 {
  RNG.with(|rng| {
    let mut r: f32 = rng.borrow_mut().random();
    r = r.clamp(0.0, 1.0 - f32::EPSILON * 2.0);

    r * (max - min) + min
  })
}
