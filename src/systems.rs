use specs::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use ggez::*;

use crate::components::*;

pub struct MovementSystem {
  pub width: u32,
  pub height: u32,
}

impl<'a> System<'a> for MovementSystem {
  type SystemData = (
    WriteStorage<'a, VelocityComponent>,
    WriteStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (mut vel, mut pos): Self::SystemData) {
    for (vel, pos) in (&mut vel, &mut pos).join() {
      // let m = 1_f32 * vel.r.cos();
      // pos.x += vel.x * 0.1;
      let x = pos.x + (vel.x * 0.1);
      let y = pos.y + (vel.y * 0.1);

      let clamped_x = x.min(self.width as f32).max(0_f32);
      let clamped_y = y.min(self.height as f32).max(0_f32);

      pos.x = clamped_x;
      pos.y = clamped_y;
    }
  }
}

pub struct RenderingSystem<'a> {
  pub ctx: &'a mut ggez::Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
  type SystemData = (
    ReadStorage<'a, PositionComponent>,
    ReadStorage<'a, ShapeComponent>,
  );

  fn run(&mut self, (pos, shape): Self::SystemData) {
    for (pos, shape) in (&pos, &shape).join() {
      match shape.variant {
        Shape::Circle(radius, draw_mode) => {
          graphics::circle(self.ctx, draw_mode, pos.to_point(), radius, 0.1).unwrap()
        }
        // Shape::Line(x1, y1, x2, y2) => panic!(""),
      }
    }
  }
}

pub struct ControllableSystem {
  pub rv: u32
};

impl<'a> System<'a> for ControllableSystem {
  type SystemData = (
    Read<'a, Input>,
    WriteStorage<'a, VelocityComponent>,
    ReadStorage<'a, ControllableComponent>,
  );

  fn run(&mut self, (input, mut vel, ctrled): Self::SystemData) {
    (&mut vel, &ctrled).join().for_each(|(vel, _ctrled)| {

      // if input.left {
      //   vel.r = vel.r - self.rv * dt end
      // }

      // if input.left {
      //   vel.x = -30.0;
      // } else if input.right {
      //   vel.x = 30.0;
      // } else {
      //   vel.x = 0.0
      // }

      // if input.up {
      //   vel.y = -30.0;
      // } else if input.down {
      //   vel.y = 30.0;
      // } else {
      //   vel.y = 0.0
      // }
    });
  }
}
