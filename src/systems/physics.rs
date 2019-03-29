use crate::components::*;
use crate::resources::*;
use ncollide2d::events::ContactEvent;
use nphysics2d::object::{Collider, RigidBody};
use nphysics2d::world::ColliderWorld;
use shrev::EventChannel;
use specs::world::*;
use specs::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
  type SystemData = (
    Read<'a, UpdateTime>,
    Write<'a, PhysicsWorld>,
    Write<'a, CollisionEvents>,
  );

  fn run(&mut self, (update_time, mut physics_world, mut collision_events): Self::SystemData) {
    // Step physics world
    let dt_seconds = update_time.0.subsec_nanos() as f32 / 1_000_000_000.0;
    physics_world.set_timestep(dt_seconds);
    physics_world.step();

    // Resolve collisions and create collision events
    let collider_world: &ColliderWorld<f32> = physics_world.collider_world();

    let contact_events = collider_world.contact_events();

    let colliders: Vec<(&Collider<f32>, &Collider<f32>)> = contact_events
      .iter()
      .flat_map(|contact_event| match contact_event {
        ContactEvent::Started(handle1, handle2) => collider_world
          .contact_pair(*handle1, *handle2, true)
          .map(|(collider1, collider2, _, _)| (collider1, collider2)),
        ContactEvent::Stopped(_, _) => None,
      })
      .collect();

    for (collider1, collider2) in colliders {
      if let Some(body) = physics_world.rigid_body(collider2.body()) {
        let rb_position = body.position().translation;

        let collider_type_1 = collider1.name().parse::<ColliderType>().unwrap();
        let collider_type_2 = collider2.name().parse::<ColliderType>().unwrap();

        collision_events.single_write(CollisionEvent {
          collider1: collider_type_1,
          collider2: collider_type_2,
          x: rb_position.x,
          y: rb_position.y,
        })
      }
    }
  }
}
