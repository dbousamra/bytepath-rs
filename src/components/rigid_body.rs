use nphysics2d::object::BodyHandle;
use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct RigidBodyComponent {
  pub handle: BodyHandle,
}
