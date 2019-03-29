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
    Write<'a, PhysicsWorld>,
  );

  fn run(
    &mut self,
    (entities, lazy, game_settings, mut spawn_info, mut physics_world): Self::SystemData,
  ) {
    let now = Instant::now();

    if now.duration_since(spawn_info.ammo_last) > spawn_info.ammo_every {
      create_ammo(&entities, &lazy, &game_settings, &mut physics_world);
      spawn_info.ammo_last = now;
    }
  }
}
