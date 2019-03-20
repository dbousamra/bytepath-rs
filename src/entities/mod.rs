use crate::components::*;
use crate::resources::*;

use ggez::Context;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::math::Velocity;
use nphysics2d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use specs::world::Builder;
use specs::{Entity, World};

pub fn create_rigid_body(
  specs_world: &mut World,
  x: f32,
  y: f32,
  velocity: f32,
  angle: f32,
  radius: f32,
) -> BodyHandle {
  let mut physics_world = specs_world.write_resource::<PhysicsWorld>();

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

pub fn create_player(ctx: &mut Context, specs_world: &mut World) -> Entity {
  let x = 400.0;
  let y = 300.0;
  let velocity = 20.0;
  let angle = 0.5;
  let radius = 20.0;

  let rigid_body_handle = create_rigid_body(specs_world, x, y, velocity, angle, radius);
  let rigid_body_component = RigidBodyComponent {
    handle: rigid_body_handle,
  };

  let position_component = PositionComponent { x, y, angle };

  let mesh = ggez::graphics::MeshBuilder::new()
    .polygon(
      ggez::graphics::DrawMode::Line(1.0),
      &[
        ggez::graphics::Point2::new(radius, 0.0),
        ggez::graphics::Point2::new(radius / 2.0, -radius / 2.0),
        ggez::graphics::Point2::new(-radius / 2.0, -radius / 2.0),
        ggez::graphics::Point2::new(-radius, 0.0),
        ggez::graphics::Point2::new(-radius / 2.0, radius / 2.0),
        ggez::graphics::Point2::new(radius / 2.0, radius / 2.0),
      ],
    )
    .polygon(
      ggez::graphics::DrawMode::Line(1.0),
      &[
        ggez::graphics::Point2::new(radius / 2.0, -radius / 2.0),
        ggez::graphics::Point2::new(0.0, -radius),
        ggez::graphics::Point2::new(-radius - radius / 2.0, -radius),
        ggez::graphics::Point2::new(-3.0 * radius / 4.0, -radius / 4.0),
        ggez::graphics::Point2::new(-radius / 2.0, -radius / 2.0),
      ],
    )
    .polygon(
      ggez::graphics::DrawMode::Line(1.0),
      &[
        ggez::graphics::Point2::new(radius / 2.0, radius / 2.0),
        ggez::graphics::Point2::new(-radius / 2.0, radius / 2.0),
        ggez::graphics::Point2::new(-3.0 * radius / 4.0, radius / 4.0),
        ggez::graphics::Point2::new(-radius - radius / 2.0, radius),
        ggez::graphics::Point2::new(0.0, radius),
      ],
    )
    .build(ctx)
    .unwrap();

  let mesh_component = MeshComponent { mesh };

  let controllable_component = ControllableComponent;

  specs_world
    .create_entity()
    .with(mesh_component)
    .with(position_component)
    .with(rigid_body_component)
    .with(controllable_component)
    .build()
}
