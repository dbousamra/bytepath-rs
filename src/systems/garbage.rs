use nphysics2d::object::{BodyHandle, RigidBody};
use specs::world::*;
use specs::*;

use crate::components::*;
use crate::resources::*;

pub struct GarbageSystem;

impl<'a> System<'a> for GarbageSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Write<'a, PhysicsSim>,
    ReadStorage<'a, GarbageComponent>,
    ReadStorage<'a, RigidBodyComponent>,
  );

  fn run(&mut self, (entities, mut physics, garbage, rigid_body): Self::SystemData) {
    for (entity, garbage) in (&entities, &garbage).join() {
      if !garbage.is_alive {
        entities.delete(entity).unwrap();
        let rigid_body_component: Option<&RigidBodyComponent> = rigid_body.get(entity);
        if let Some(body) = rigid_body_component {
          physics.world.remove_bodies(&[body.handle]);
          physics.bodies.remove(&body.handle);
        }
      }
    }
  }
}
