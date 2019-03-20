use nphysics2d::object::{BodyHandle, RigidBody};
use specs::*;

use crate::components::*;
use crate::resources::*;

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
  type SystemData = (
    Write<'a, PhysicsWorld>,
    ReadStorage<'a, RigidBodyComponent>,
    WriteStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (mut physics_world, rb, mut position): Self::SystemData) {
    for (rb, position) in (&rb, &mut position).join() {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let rb_position = body.position().translation;
      position.x = rb_position.x;
      position.y = rb_position.y;
    }
  }
}
