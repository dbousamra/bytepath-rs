use nalgebra::{Isometry2, Vector2};
use nphysics2d::object::{BodyHandle, RigidBody};
use rand::Rng;
use specs::world::*;
use specs::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;

pub struct ControllableSystem;

impl<'a> System<'a> for ControllableSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, LazyUpdate>,
    Read<'a, Input>,
    Write<'a, PhysicsWorld>,
    WriteStorage<'a, RigidBodyComponent>,
    ReadStorage<'a, ControllableComponent>,
  );

  fn run(&mut self, (entities, lazy, input, mut physics_world, mut rb, ctrled): Self::SystemData) {
    (&mut rb, &ctrled).join().for_each(|(rb, _ctrled)| {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let pos = body.position().translation;
      let angle = body.position().rotation.angle();
      let v = 250.0;

      let new_angle = if input.left {
        angle - 0.05
      } else if input.right {
        angle + 0.05
      } else {
        angle
      };

      body.set_position(Isometry2::new(Vector2::new(pos.x, pos.y), new_angle));
      body.set_linear_velocity(Vector2::new(new_angle.cos() * v, new_angle.sin() * v));

      let mut rng = rand::thread_rng();

      if input.attack {
        for _ in 0..rng.gen_range(8, 12) {
          create_death_explosion(&entities, &lazy, &mut physics_world, pos.x, pos.y);
        }
      }
    });
  }
}
