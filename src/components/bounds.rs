use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct BoundsComponent {
  pub x_min: f32,
  pub x_max: f32,
  pub y_min: f32,
  pub y_max: f32,
}
