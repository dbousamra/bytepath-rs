use crate::components::*;
use crate::entities::*;
use crate::resources::*;

use nphysics2d::object::{Collider, RigidBody};
use nphysics2d::world::ColliderWorld;
use rand::Rng;
use specs::world::*;
use specs::*;
use std::time::Instant;

#[derive(Default)]
pub struct CollisionSystem {
  reader: Option<ReaderId<CollisionEvent>>,
}

impl<'a> System<'a> for CollisionSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, LazyUpdate>,
    Write<'a, PhysicsWorld>,
    Read<'a, CollisionEvents>,
  );

  fn run(&mut self, (entities, lazy, mut physics_world, collision_events): Self::SystemData) {
    for event in collision_events.read(&mut self.reader.as_mut().unwrap()) {
      let mut rng = rand::thread_rng();

      match (&event.collider1, &event.collider2) {
        (ColliderType::Player, ColliderType::Ammo) | (ColliderType::Ammo, ColliderType::Player) => {
          println!("Player and ammmo collided")
        }
        (ColliderType::PlayerProjectile, ColliderType::Ammo)
        | (ColliderType::Ammo, ColliderType::PlayerProjectile) => {
          for _ in 0..rng.gen_range(4, 20) {
            create_death_explosion(&entities, &lazy, &mut physics_world, event.x, event.y)
          }
        }
        _ => {}
      };
    }
  }

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.reader = Some(res.fetch_mut::<CollisionEvents>().register_reader());
  }
}
