use crate::resources::assets::effects::PlayerEffectLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::effect::PlayerEffect;
use crate::resources::{distance, EffectUpdateProps};

#[derive(Clone, Debug)]
pub struct PlayerSlow {
  pub target_speed: f32,
  pub effect: PlayerEffect,
}

impl PlayerSlow {
  pub fn new(target: &HeroWrapper, caster_id: u64) -> Self {
    let player = target.player();
    let mut s = Self {
      target_speed: player.speed,
      effect: PlayerEffect::new(0, player.id, caster_id),
    };
    s.effect.id = 0;
    s
  }
}

impl PlayerEffectLogic for PlayerSlow {
  fn enable(&mut self, player: &mut HeroWrapper) {
    player.player_mut().speed = self.target_speed * 0.25;
  }
  fn disable(&self, player: &mut HeroWrapper) {
    player.player_mut().speed = self.target_speed;
  }
  fn update(&mut self, props: &mut EffectUpdateProps<'_>) {
    let target = props.target.player();
    let caster = props.caster.entity();

    if distance(target.pos.x - caster.pos.x, target.pos.y - caster.pos.y) >= 150.0 + target.radius {
      self.effect.to_remove = true;
    }
    if target.pos.x - target.radius < 0.0 && target.pos.x > props.boundary.w + target.radius {
      self.effect.to_remove = true;
    }
  }
  fn effect(&self) -> &PlayerEffect {
    &self.effect
  }
  fn effect_mut(&mut self) -> &mut PlayerEffect {
    &mut self.effect
  }
  fn effect_id(&self) -> u64 {
    self.effect.id
  }
}
