use specs::*;

use crate::components::*;
use crate::resources::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
  type SystemData = (Read<'a, UpdateTime>, Write<'a, PhysicsWorld>);

  fn run(&mut self, (update_time, mut physics_world): Self::SystemData) {
    physics_world.set_timestep(update_time.0);
    physics_world.step();
  }
}
