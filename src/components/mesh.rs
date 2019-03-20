use specs::{Component, VecStorage};

use ggez::graphics::Mesh;

#[derive(Debug)]
pub struct MeshComponent {
  pub mesh: Mesh,
}

impl Component for MeshComponent {
  type Storage = VecStorage<Self>;
}
