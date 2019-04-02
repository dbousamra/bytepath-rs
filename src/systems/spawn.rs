use specs::world::*;
use specs::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;

use std::time::{Duration, Instant};

pub struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, LazyUpdate>,
    Read<'a, GameSettings>,
    Write<'a, SpawnInfo>,
    Write<'a, PhysicsSim>,
  );

  fn run(
    &mut self,
    (entities, lazy, game_settings, mut spawn_info, mut physics): Self::SystemData,
  ) {
    let now = Instant::now();

    if now.duration_since(spawn_info.ammo_last) > spawn_info.ammo_every
      && spawn_info.ammo_count < spawn_info.ammo_max
    {
      create_ammo(&entities, &lazy, &game_settings, &mut physics);
      spawn_info.ammo_last = now;
      spawn_info.ammo_count += 1;
    }
  }
}
