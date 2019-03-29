use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct PositionComponent {
  pub x: f32,
  pub y: f32,
  pub angle: f32,
}
