use nphysics2d::object::{BodyHandle, RigidBody};
use specs::*;

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
      let timestep = physics_world.timestep();
      let rv = std::f32::consts::PI * 1.66;

      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let v = body.velocity().as_vector().norm();

      if input.left {
        rb.angle = rb.angle - (rv * timestep * 40.0);
      } else if input.right {
        rb.angle = rb.angle + (rv * timestep * 40.0);
      }

      let angle_in_radian = rb.angle / 180.0 * std::f32::consts::PI;
      body.set_linear_velocity(Vector2::new(
        angle_in_radian.sin() * v,
        angle_in_radian.cos() * v,
      ));
    });
  }
}