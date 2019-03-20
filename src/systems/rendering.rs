use specs::{ReadStorage, System};

use ggez::Context;

use crate::components::*;

pub struct RenderingSystem<'a> {
  pub ctx: &'a mut ggez::Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
  type SystemData = ReadStorage<'a, MeshComponent>;

  fn run(&mut self, mesh: Self::SystemData) {
    use specs::Join;

    for mesh in mesh.join() {
      let dest = ggez::graphics::Point2::new(400.0, 300.0);
      let rotation = 0.0;

      ggez::graphics::draw(self.ctx, &mesh.mesh, dest, rotation);
    }
  }
}
