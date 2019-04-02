use nphysics2d::object::{BodyHandle, RigidBody};
use specs::world::*;
use specs::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;

pub struct PickupSystem;

impl<'a> System<'a> for PickupSystem {
  type SystemData = (
    ReadStorage<'a, PickupEvents>,
  );

  fn run(
    &mut self,
    (pickups): Self::SystemData,
  ) {
    
  }
}
