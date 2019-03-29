use ncollide2d::world::CollisionGroups;
use nphysics2d::object::ColliderHandle;

use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct CollisionComponent {
  pub handle: ColliderHandle,
}
