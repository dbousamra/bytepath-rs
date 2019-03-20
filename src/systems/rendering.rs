use ggez::Context;
use specs::*;

use crate::components::*;

pub struct RenderingSystem<'a> {
  pub ctx: &'a mut ggez::Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
  type SystemData = (
    ReadStorage<'a, MeshComponent>,
    ReadStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (mesh, position): Self::SystemData) {
    use specs::Join;
    for (mesh, position) in (&mesh, &position).join() {
      ggez::graphics::draw(
        self.ctx,
        &mesh.mesh,
        ggez::graphics::Point2::new(position.x, position.y),
        position.angle,
      )
      .unwrap()
    }
  }
}
