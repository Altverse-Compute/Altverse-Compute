use crate::flat::altverse_server::{PackedEntity, PackedPlayer};
use crate::resources::assets::entity::EntityWrapper;
use crate::resources::assets::hero::HeroWrapper;
use flatbuffers::{FlatBufferBuilder, TableFinishedWIPOffset, WIPOffset};

pub fn build_join_player(
  builder: &mut FlatBufferBuilder,
  hero_wrapper: &HeroWrapper,
) -> TableFinishedWIPOffset {
  let player = hero_wrapper.player();
  let name = builder.try_create_string(player.name)?;
  let start = builder.start_table();
  builder.push_slot_always(PackedPlayer::VT_NAME, name);
  builder.push_slot_always(PackedPlayer::VT_ID, player.id);
  builder.push_slot_always(PackedPlayer::VT_X, player.pos.x);
  builder.push_slot_always(PackedPlayer::VT_Y, player.pos.y);
  builder.push_slot_always(PackedPlayer::VT_RADIUS, player.radius);
  builder.push_slot_always(PackedPlayer::VT_SPEED, player.speed);
  builder.push_slot_always(PackedPlayer::VT_ENERGY, player.energy);
  builder.push_slot_always(PackedPlayer::VT_MAX_ENERGY, player.max_energy);
  builder.push_slot_always(PackedPlayer::VT_DEATH_TIMER, player.death_timer);
  builder.push_slot_always(PackedPlayer::VT_STATE, player.state);
  builder.push_slot_always(PackedPlayer::VT_STATE_META, player.state_meta);
  builder.push_slot_always(PackedPlayer::VT_AREA, player.area);
  builder.push_slot_always(PackedPlayer::VT_WORLD, player.world);
  builder.push_slot_always(PackedPlayer::VT_DIED, player.downed);
  builder.push_slot_always(PackedPlayer::VT_HERO, player.hero);
  builder.end_table(start)
}

pub fn build_entity(
  builder: &mut FlatBufferBuilder,
  entity_wrapper: &EntityWrapper,
) -> WIPOffset<TableFinishedWIPOffset> {
  let entity = entity_wrapper.entity();
  let start = builder.start_table();
  builder.push_slot_always(PackedEntity::VT_TYPE_ID, entity.type_id);
  builder.end_table(start)
}
