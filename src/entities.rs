use ggez::Context;
use specs::{Builder, World};

use ggez::graphics::{DrawMode, Point2};
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, ConvexPolygon, Cuboid, ShapeHandle};
use nphysics2d::math::{Inertia, Velocity};
use nphysics2d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBody, RigidBodyDesc};

use crate::components::*;
use crate::resources::*;

pub fn create_rigid_body(
  world: &mut World,
  x: f32,
  y: f32,
  velocity: f32,
  radius: f32,
  angle: f32,
) -> BodyHandle {
  let mut physics_world = world.write_resource::<PhysicsWorld>();

  let cuboid = ShapeHandle::new(Ball::new(radius));
  let collider_desc = ColliderDesc::new(cuboid);

  RigidBodyDesc::new()
    .collider(&collider_desc)
    .position(Isometry2::new(Vector2::new(x, y as f32), angle))
    .velocity(Velocity::linear(velocity, 0.0))
    .status(BodyStatus::Dynamic)
    .build(&mut physics_world)
    .handle()
}

pub fn create_player(_ctx: &mut Context, world: &mut World, width: f32, height: f32) {
  let x = width / 2.0;
  let y = height / 2.0;
  let v = 0.0;
  let radius = 10.0;
  let angle = 0.0;
  let line_thickness = 1.0;

  let x_min = 0.0 + radius + (line_thickness * 8.0);
  let x_max = width - radius - (line_thickness * 8.0);
  let y_min = 0.0 + radius + (line_thickness * 8.0);
  let y_max = height - radius - (line_thickness * 8.0);

  let handle = create_rigid_body(world, x, y, v, radius, angle);
  world
    .create_entity()
    .with(RigidBodyComponent {
      handle: handle,
      // angle: angle,
    })
    .with(PositionComponent { x, y, angle })
    .with(BoundsComponent {
      x_min,
      x_max,
      y_min,
      y_max,
    })
    .with(ControllableComponent)
    .with(ShapesComponent {
      shapes: vec![
        Shape::Circle(
          Vector2::new(0.0, 0.0),
          radius,
          DrawMode::Line(line_thickness),
        ),
        Shape::Line(Vector2::new(10.0, 10.0), 10.0, 1.0),
      ],
    })
    .build();
}
