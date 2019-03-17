extern crate nalgebra;

use ggez::graphics::{DrawMode, Point2, Rect};
use nalgebra::geometry::*;
use nalgebra::Vector2;
use nphysics2d::algebra::*;
use nphysics2d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBody, RigidBodyDesc};
use specs::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use ggez::*;

use crate::components::*;
use crate::resources::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
  type SystemData = (Read<'a, UpdateTime>, Write<'a, PhysicsWorld>);

  fn run(&mut self, (update_time, mut physics_world): Self::SystemData) {
    physics_world.set_timestep(update_time.0);
    physics_world.step();
  }
}

pub struct BoundsSystem;

impl<'a> System<'a> for BoundsSystem {
  type SystemData = (
    Write<'a, PhysicsWorld>,
    WriteStorage<'a, RigidBodyComponent>,
    ReadStorage<'a, BoundsComponent>,
  );

  fn run(&mut self, (mut physics_world, mut rb, bounds): Self::SystemData) {
    for (rb, bounds) in (&mut rb, &bounds).join() {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let position = body.position().translation;
      let angle = body.position().rotation.angle();

      let clamped_x = position.x.min(bounds.x_max).max(bounds.x_min);
      let clamped_y = position.y.min(bounds.y_max).max(bounds.y_min);

      body.set_position(Isometry2::new(Vector2::new(clamped_x, clamped_y), angle));
    }
  }
}

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
  type SystemData = (
    Write<'a, PhysicsWorld>,
    ReadStorage<'a, RigidBodyComponent>,
    WriteStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (mut physics_world, rb, mut position): Self::SystemData) {
    for (rb, position) in (&rb, &mut position).join() {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let rb_position = body.position().translation;
      position.x = rb_position.x;
      position.y = rb_position.y;
      // position.angle = rb.angle;
    }
  }
}

pub struct RenderingSystem<'a> {
  pub ctx: &'a mut ggez::Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
  type SystemData = (
    ReadStorage<'a, ShapesComponent>,
    ReadStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (shape, position): Self::SystemData) {
    for (shape, position) in (&shape, &position).join() {
      for shape in shape.shapes.iter() {
        match shape {
          Shape::Circle(offset, radius, mode) => {
            let pos = Vector2::new(position.x, position.y) + offset;
            let point = Point2::new(pos.x, position.y);
            graphics::circle(self.ctx, *mode, point, *radius, 0.1).unwrap()
          }
          Shape::Line(offset, length, thickness) => {
            let p1 = Point2::new(position.x, position.y);
            let p2 = Point2::new(p1.x + 5.0, p1.y + 5.0);
            graphics::line(self.ctx, &[p1, p2], *thickness).unwrap();
          }
        }
      }
    }
  }
}

pub struct ControllableSystem;

impl<'a> System<'a> for ControllableSystem {
  type SystemData = (
    Read<'a, Input>,
    Write<'a, PhysicsWorld>,
    WriteStorage<'a, RigidBodyComponent>,
    ReadStorage<'a, ControllableComponent>,
  );

  fn run(&mut self, (input, mut physics_world, mut rb, ctrled): Self::SystemData) {
    (&mut rb, &ctrled).join().for_each(|(rb, _ctrled)| {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(rb.handle)
        .expect("Rigid body in specs does not exist in physics world");

      let pos = body.position().translation;
      let angle = body.position().rotation.angle();
      let v = 200.0;

      let new_angle = if input.left {
        angle - 0.05
      } else if input.right {
        angle + 0.05
      } else {
        angle
      };

      body.set_position(Isometry2::new(Vector2::new(pos.x, pos.y), new_angle));
      body.set_linear_velocity(Vector2::new(new_angle.sin() * v, new_angle.cos() * v));
    });
  }
}

pub struct FollowEntitySystem;

impl<'a> System<'a> for FollowEntitySystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, FollowsEntityComponent>,
    WriteStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (entity, follows, mut position): Self::SystemData) {
    for (entity, follows) in (&entity, &follows).join() {
      let new_position: PositionComponent = position.get(follows.target).cloned().unwrap();
      *position.get_mut(entity).unwrap() = new_position;
    }
  }
}
