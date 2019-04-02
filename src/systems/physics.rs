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
    Write<'a, PhysicsSim>,
    Write<'a, CollisionEvents>,
  );

  fn run(&mut self, (update_time, mut physics, mut collision_events): Self::SystemData) {
    // Step physics world
    let dt_seconds = update_time.0.subsec_nanos() as f32 / 1_000_000_000.0;
    physics.world.set_timestep(dt_seconds);
    physics.world.step();

    // Resolve collisions and create collision events
    let collider_world: &ColliderWorld<f32> = physics.world.collider_world();

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
      let collider1_body_handle = collider1.body();
      let collider2_body_handle = collider2.body();

      let collider1_entity = physics.bodies.get(&collider1_body_handle);
      let collider2_entity = physics.bodies.get(&collider2_body_handle);

      let joined = (
        collider1_entity,
        collider2_entity,
        physics.world.rigid_body(collider2_body_handle),
      );

      match joined {
        (Some(e1), Some(e2), Some(body)) => {
          let rb_position = body.position().translation;

          let collider_type_1 = collider1.name().parse::<ColliderType>().unwrap();
          let collider_type_2 = collider2.name().parse::<ColliderType>().unwrap();

          let collision_type = match (collider_type_1, collider_type_2) {
            (ColliderType::Player, ColliderType::Ammo) => Some(CollisionType::PlayerAmmo {
              player: *e1,
              ammo: *e2,
            }),
            (ColliderType::Ammo, ColliderType::Player) => Some(CollisionType::PlayerAmmo {
              player: *e2,
              ammo: *e1,
            }),
            (_, _) => None,
          };

          if let Some(ct) = collision_type {
            collision_events.single_write(CollisionEvent {
              collision_type: ct,
              x: rb_position.x,
              y: rb_position.y,
            })
          }
        }
        (_, _, _) => {}
      };
    }
  }
}
