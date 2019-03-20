use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct PositionComponent {
  pub x: f32,
  pub y: f32,
}

impl Component for PositionComponent {
  type Storage = VecStorage<Self>;
}
