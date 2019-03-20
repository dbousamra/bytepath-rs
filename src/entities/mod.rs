use crate::components::*;

use ggez::Context;
use specs::world::Builder;
use specs::{Entity, World};

pub fn create_player(ctx: &mut Context, specs_world: &mut World) -> Entity {
  let w = 20.0;

  let mesh = ggez::graphics::MeshBuilder::new()
    .polygon(
      ggez::graphics::DrawMode::Line(1.0),
      &[
        ggez::graphics::Point2::new(w, 0.0),
        ggez::graphics::Point2::new(w / 2.0, -w / 2.0),
        ggez::graphics::Point2::new(-w / 2.0, -w / 2.0),
        ggez::graphics::Point2::new(-w, 0.0),
        ggez::graphics::Point2::new(-w / 2.0, w / 2.0),
        ggez::graphics::Point2::new(w / 2.0, w / 2.0),
      ],
    )
    .polygon(
      ggez::graphics::DrawMode::Line(1.0),
      &[
        ggez::graphics::Point2::new(w / 2.0, -w / 2.0),
        ggez::graphics::Point2::new(0.0, -w),
        ggez::graphics::Point2::new(-w - w / 2.0, -w),
        ggez::graphics::Point2::new(-3.0 * w / 4.0, -w / 4.0),
        ggez::graphics::Point2::new(-w / 2.0, -w / 2.0),
      ],
    )
    .polygon(
      ggez::graphics::DrawMode::Line(1.0),
      &[
        ggez::graphics::Point2::new(w / 2.0, w / 2.0),
        ggez::graphics::Point2::new(-w / 2.0, w / 2.0),
        ggez::graphics::Point2::new(-3.0 * w / 4.0, w / 4.0),
        ggez::graphics::Point2::new(-w - w / 2.0, w),
        ggez::graphics::Point2::new(0.0, w),
      ],
    )
    .build(ctx)
    .unwrap();

  let mesh_component = MeshComponent { mesh };

  specs_world.create_entity().with(mesh_component).build()
}
