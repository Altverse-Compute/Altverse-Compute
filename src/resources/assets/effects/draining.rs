use crate::resources::assets::effects::PlayerEffectLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::effect::PlayerEffect;
use crate::resources::{EffectUpdateProps, distance};

#[derive(Clone, Debug)]
pub struct PlayerDraining {
  pub effect: PlayerEffect,
}

impl PlayerDraining {
  pub fn new(target: &HeroWrapper, caster_id: u64) -> Self {
    let player = target.player();
    let mut s = Self {
      effect: PlayerEffect::new(1, player.id, caster_id),
    };
    s.effect.id = 1;
    s
  }
}

impl PlayerEffectLogic for PlayerDraining {
  fn enable(&mut self, _: &mut HeroWrapper) {}
  fn disable(&self, _: &mut HeroWrapper) {}
  fn update(&mut self, props: &mut EffectUpdateProps<'_>) {
    let target = props.target.player_mut();
    let caster = props.caster.entity();

    target.energy -= 16.0 * props.delta / 1000.0;
    target.changed_energy();

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
