use specs::{Component, VecStorage};

use ggez::graphics::{DrawParam, MeshBuilder};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct MeshComponent {
  pub mesh: MeshBuilder,
  pub draw_param: DrawParam,
}
