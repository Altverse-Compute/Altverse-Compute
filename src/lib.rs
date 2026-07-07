// #[deny(clippy::all)]
use crate::builder::build_packages;
use crate::bus::{EventBus, NetworkBus};
use crate::config::Config;
use crate::fbs::{Chat, Package, Role};
use crate::managers::player::PlayersManager;
use crate::managers::world::WorldsManager;
use crate::props::EngineProps;
use crate::resources::UpdateProps;
use crate::resources::utils::input::Input;
use crate::resources::utils::join::JoinProps;
use chrono::Utc;
use lazy_static::lazy_static;
use napi::bindgen_prelude::{Buffer, Null};
use napi::bindgen_prelude::{Function, Uint8ArraySlice};
use napi::bindgen_prelude::{JsObjectValue, Object};
use napi::{Env, Error};
use napi_derive::napi;
use std::sync::Mutex;

pub mod flat {
  include!("proto/gen/flat/game_generated.rs");
}

mod builder;
mod bus;
mod config;
mod fbs;
mod managers;
mod props;
mod resources;

lazy_static! {
  pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

#[napi]
pub struct ComputeEngine {
  players_manager: PlayersManager,
  worlds_manager: WorldsManager,
  network_bus: NetworkBus,
  event_bus: EventBus,
  proto_buffer: Vec<u8>,

  last_timestamp: i64,
  player_death_callback: Option<Function<'static, i64, Null>>,
}

#[napi]
impl ComputeEngine {
  #[napi(constructor)]
  pub fn new(props: &EngineProps) -> Result<Self, Error> {
    // let worlds = props.load_worlds()?;
    let config = props.load_config()?;

    *CONFIG.lock().unwrap() = config.clone();

    Ok(Self {
      players_manager: PlayersManager::new(),
      worlds_manager: WorldsManager::new(props),
      network_bus: NetworkBus::new(),
      last_timestamp: Utc::now().timestamp_millis(),
      proto_buffer: Vec::with_capacity(1024),
      event_bus: EventBus::new(),
      player_death_callback: None,
    })
  }

  #[napi]
  pub fn join(&mut self, player_props: &JoinProps) -> Result<(), Error> {
    self.network_bus.add_client(player_props.id as u64);
    self.players_manager.join(
      player_props,
      &mut self.worlds_manager.worlds,
      &mut self.network_bus,
    )?;
    Ok(())
  }

  #[napi]
  pub fn leave(&mut self, player_id: i64) {
    self.players_manager.leave(
      player_id as u64,
      &mut self.worlds_manager.worlds,
      &mut self.network_bus,
    );
    self.network_bus.remove_client(player_id as u64);
  }

  #[napi]
  pub fn chat_message(&mut self, content: String, id: i64) {
    if let Some(hero) = self.players_manager.get_player(id as u64) {
      self.network_bus.add_global_package(Package::Chat(Chat {
        id: id.try_into().unwrap(),
        content,
        author: hero.player().name.clone(),
        role: Role::User,
        world: hero.player().world.clone(),
      }))
    }
  }

  #[napi]
  pub fn input(&mut self, id: i64, input: &Input) {
    self.network_bus.accept_input(id as u64, input);
  }

  #[napi]
  pub fn on_player_death(&mut self, callback: Function<'static, i64, Null>) {
    self.player_death_callback = Some(callback);
  }

  #[napi]
  pub fn update(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let config = CONFIG.lock().unwrap();

    let time = Utc::now().timestamp_millis();
    let delta = (time - self.last_timestamp) as f32;
    self.last_timestamp = time;
    let time_fix = delta as f32 / (1000.0 / 30.0);

    let update_props = UpdateProps { delta, time_fix };

    self.worlds_manager.update(
      &update_props,
      &mut self.players_manager,
      &mut self.network_bus,
      &mut self.event_bus,
    );
    self
      .worlds_manager
      .process_warps(&mut self.players_manager, &config, &mut self.network_bus);
    self.players_manager.update_behavior(
      &update_props,
      &mut self.worlds_manager.worlds,
      &mut self.network_bus,
      &mut self.event_bus,
    );
    self.players_manager.snapshot_end(&mut self.network_bus);

    if let Some(callback) = self.player_death_callback {
      for id in self.players_manager.check_players_to_remove() {
        self.leave(id.try_into().unwrap());
        let _ = callback.call(id.try_into().unwrap());
      }
    }

    self.packages_as_napi(env)
  }

  fn packages_as_napi(&mut self, env: &Env) -> Result<Object<'_>, Error> {
    let mut object = Object::new(env)?;

    for (key, value) in self.network_bus.direct_clients.iter_mut() {
      let result = build_packages(value, &mut self.players_manager, &mut self.worlds_manager);
      value.flat_builder.finish(result, None);
      let uint8 = Buffer::from(value.flat_builder.finished_data());
      object.set(key.to_string(), uint8);
      value.packages.clear();
      value.flat_builder.reset();
    }

    self.network_bus.clear_packages();
    self.players_manager.clean_changes();
    self.worlds_manager.clean_changes();

    Ok(object)
  }
}
