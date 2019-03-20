use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Mesh {
  pub x: f32,
  pub y: f32,
}

impl Component for Mesh {
  type Storage = VecStorage<Self>;
}
