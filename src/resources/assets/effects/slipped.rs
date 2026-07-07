use crate::resources::assets::effects::PlayerEffectLogic;
use crate::resources::assets::hero::HeroWrapper;
use crate::resources::effect::PlayerEffect;
use crate::resources::EffectUpdateProps;

#[derive(Clone, Debug)]
pub struct PlayerSlipped {
  pub effect: PlayerEffect,
  time: f32,
  original_speed: f32,
}

impl PlayerSlipped {
  pub fn new(target: &HeroWrapper, caster_id: u64) -> Self {
    let player = target.player();
    let mut s = Self {
      effect: PlayerEffect::new(2, player.id, caster_id),
      time: 100.0,
      original_speed: player.speed,
    };
    s
  }
}

impl PlayerEffectLogic for PlayerSlipped {
  fn enable(&mut self, hero: &mut HeroWrapper) {
    hero.player_mut().speed = self.original_speed * 2.0;
  }
  fn disable(&self, hero: &mut HeroWrapper) {
    hero.player_mut().speed = self.original_speed;
  }
  fn update(&mut self, props: &mut EffectUpdateProps<'_>) {
    let target = props.target.player_mut();
    let caster = props.caster.entity();
    self.time -= props.delta;
    if self.time < 0.0 {
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
