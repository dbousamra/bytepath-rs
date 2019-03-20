use specs::*;

use nalgebra::{Isometry2, Vector2};
use nphysics2d::object::{BodyHandle, RigidBody};

use crate::components::*;
use crate::resources::*;

pub struct ControllableSystem;

impl<'a> System<'a> for ControllableSystem {
  type SystemData = (
    Read<'a, Input>,
    Write<'a, PhysicsWorld>,
    WriteStorage<'a, RigidBodyComponent>,
    ReadStorage<'a, ControllableComponent>,
  );

  fn run(&mut self, (input, mut physics_world, mut rb, ctrled): Self::SystemData) {
    (&mut rb, &ctrled).join().for_each(|(rb, _ctrled)| {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let pos = body.position().translation;
      let angle = body.position().rotation.angle();
      let v = 200.0;

      let new_angle = if input.left {
        angle - 0.05
      } else if input.right {
        angle + 0.05
      } else {
        angle
      };

      body.set_position(Isometry2::new(Vector2::new(pos.x, pos.y), new_angle));
      body.set_linear_velocity(Vector2::new(new_angle.cos() * v, new_angle.sin() * v));
    });
  }
}
