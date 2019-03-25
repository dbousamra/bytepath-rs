use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct GarbageComponent {
  pub is_alive: bool,
}

impl Default for GarbageComponent {
  fn default() -> GarbageComponent {
    GarbageComponent { is_alive: true }
  }
}
