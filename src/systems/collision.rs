use crate::components::*;
use crate::entities::*;
use crate::resources::*;

use nphysics2d::object::{Collider, RigidBody};
use nphysics2d::world::ColliderWorld;
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
    Write<'a, PhysicsSim>,
    Read<'a, CollisionEvents>,
    WriteStorage<'a, GarbageComponent>,
  );

  fn run(
    &mut self,
    (entities, lazy, mut physics, collision_events, mut garbage): Self::SystemData,
  ) {
    for event in collision_events.read(&mut self.reader.as_mut().unwrap()) {
      match event.collision_type {
        CollisionType::PlayerAmmo { player: _, ammo } => {
          garbage.get_mut(ammo).map(|g| g.is_alive = false);
          create_death_explosion(&entities, &lazy, &mut physics, event.x, event.y)
        }
      }
    }
  }

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.reader = Some(res.fetch_mut::<CollisionEvents>().register_reader());
  }
}
