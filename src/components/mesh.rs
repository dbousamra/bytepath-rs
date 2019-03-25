use specs::{Component, VecStorage};

use ggez::graphics::{DrawParam, MeshBuilder};

#[derive(Debug)]
pub struct MeshComponent {
  pub mesh: MeshBuilder,
  pub draw_param: DrawParam,
}

impl Component for MeshComponent {
  type Storage = VecStorage<Self>;
}
