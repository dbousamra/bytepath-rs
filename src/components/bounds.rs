use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct BoundsComponent {
  pub x_min: u32,
  pub x_max: u32,
  pub y_min: u32,
  pub y_max: u32,
}
