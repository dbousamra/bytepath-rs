use ggez::graphics;
use ggez::Context;
use specs::*;

use crate::components::*;
use crate::resources::*;

pub struct RenderingSystem<'a> {
  pub ctx: &'a mut ggez::Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
  type SystemData = (
    ReadStorage<'a, MeshComponent>,
    ReadStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (mesh, position): Self::SystemData) {
    for (mesh, position) in (&mesh, &position).join() {
      let drawable: graphics::Mesh = mesh.mesh.build(self.ctx).unwrap();
      let dest = ggez::graphics::Point2::new(position.x, position.y);
      let angle = position.angle;

      let draw_param = graphics::DrawParam {
        dest: dest,
        rotation: angle,
        scale: mesh.draw_param.scale,
        src: mesh.draw_param.src,
        offset: mesh.draw_param.offset,
        shear: mesh.draw_param.shear,
        color: mesh.draw_param.color,
      };

      ggez::graphics::draw_ex(self.ctx, &drawable, draw_param).unwrap();
    }
  }
}
