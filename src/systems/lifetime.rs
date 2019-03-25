use specs::world::*;
use specs::*;
use std::time::Duration;

use crate::components::*;
use crate::resources::*;

pub struct LifetimeSystem;

impl<'a> System<'a> for LifetimeSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, UpdateTime>,
    WriteStorage<'a, LifetimeComponent>,
    WriteStorage<'a, GarbageComponent>,
  );

  fn run(&mut self, (entities, update_time, mut lifetime, mut garbage): Self::SystemData) {
    for (entity, lifetime, garbage) in (&entities, &mut lifetime, &mut garbage).join() {
      let new_lifetime = lifetime.duration.checked_sub(update_time.0);

      match new_lifetime {
        Some(d) => lifetime.duration = d,
        None => garbage.is_alive = false,
      };
    }
  }
}
