use specs::*;

use crate::components::*;
use crate::resources::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
  type SystemData = (Read<'a, UpdateTime>, Write<'a, PhysicsWorld>);

  fn run(&mut self, (update_time, mut physics_world): Self::SystemData) {
    let dt_seconds = update_time.0.subsec_nanos() as f32 / 1_000_000_000.0;
    physics_world.set_timestep(dt_seconds);
    physics_world.step();
  }
}
