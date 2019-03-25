use crate::components::*;
use crate::resources::*;
use crate::utils::*;

use core::ops::{Deref, DerefMut};
use ggez::graphics;
use ggez::Context;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::math::Velocity;
use nphysics2d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use rand::Rng;
use specs::world::*;
use specs::*;
use std::time::{Duration, Instant};

pub fn create_player(
  _ctx: &mut Context,
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  game_settings: &GameSettings,
  physics_world: &mut PhysicsWorld,
) -> Entity {
  let x = game_settings.width as f32 / 2.0;
  let y = game_settings.height as f32 / 2.0;
  let velocity = 0.0;
  let angle = 0.5;
  let size = 25.0;

  let cuboid = ShapeHandle::new(Ball::new(size));
  let collider_desc = ColliderDesc::new(cuboid);
  let rigid_body_handle = RigidBodyDesc::new()
    .collider(&collider_desc)
    .position(Isometry2::new(Vector2::new(x, y), angle))
    .velocity(Velocity::linear(velocity, 0.0))
    .status(BodyStatus::Dynamic)
    .build(physics_world)
    .handle();

  let rigid_body_component = RigidBodyComponent {
    handle: rigid_body_handle,
  };

  let position_component = PositionComponent { x, y, angle };

  let mesh = graphics::MeshBuilder::new()
    .polygon(
      graphics::DrawMode::Line(1.0),
      &[
        graphics::Point2::new(size, 0.0),
        graphics::Point2::new(size / 2.0, -size / 2.0),
        graphics::Point2::new(-size / 2.0, -size / 2.0),
        graphics::Point2::new(-size, 0.0),
        graphics::Point2::new(-size / 2.0, size / 2.0),
        graphics::Point2::new(size / 2.0, size / 2.0),
      ],
    )
    .polygon(
      graphics::DrawMode::Line(1.0),
      &[
        graphics::Point2::new(size / 2.0, -size / 2.0),
        graphics::Point2::new(0.0, -size),
        graphics::Point2::new(-size - size / 2.0, -size),
        graphics::Point2::new(-3.0 * size / 4.0, -size / 4.0),
        graphics::Point2::new(-size / 2.0, -size / 2.0),
      ],
    )
    .polygon(
      graphics::DrawMode::Line(1.0),
      &[
        graphics::Point2::new(size / 2.0, size / 2.0),
        graphics::Point2::new(-size / 2.0, size / 2.0),
        graphics::Point2::new(-3.0 * size / 4.0, size / 4.0),
        graphics::Point2::new(-size - size / 2.0, size),
        graphics::Point2::new(0.0, size),
      ],
    )
    .clone();

  let draw_param = graphics::DrawParam::default();

  let mesh_component = MeshComponent { mesh, draw_param };

  let controllable_component = ControllableComponent;

  let shooting_component = ShootingComponent {
    every: Duration::from_millis(250),
    last_shot_at: Instant::now(),
  };

  let garbage_component = GarbageComponent::default();

  LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(rigid_body_component)
  .with(controllable_component)
  .with(shooting_component)
  .with(garbage_component)
  .build()
}

pub fn create_projectile(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  settings: &GameSettings,
  physics_world: &mut PhysicsWorld,
  position_component: PositionComponent,
) -> Entity {
  let x = position_component.x;
  let y = position_component.y;
  let angle = position_component.angle;
  let velocity = 500.0;

  let radius = 4.0;
  let tolerance = 0.1;
  let thickness = 1.0;

  let cuboid = ShapeHandle::new(Ball::new(radius));
  let collider_desc = ColliderDesc::new(cuboid);
  let rigid_body = RigidBodyDesc::new()
    .collider(&collider_desc)
    .position(Isometry2::new(Vector2::new(x, y), angle))
    .status(BodyStatus::Dynamic)
    .build(physics_world);

  rigid_body.set_linear_velocity(Vector2::new(angle.cos() * velocity, angle.sin() * velocity));

  let rigid_body_component = RigidBodyComponent {
    handle: rigid_body.handle(),
  };

  let mesh = graphics::MeshBuilder::new()
    .circle(
      graphics::DrawMode::Line(thickness),
      graphics::Point2::origin(),
      radius,
      tolerance,
    )
    .clone();

  let draw_param = graphics::DrawParam::default();

  let mesh_component = MeshComponent { mesh, draw_param };

  let garbage_component = GarbageComponent::default();

  let bounds_component = BoundsComponent {
    x_min: 0,
    x_max: settings.width,
    y_min: 0,
    y_max: settings.height,
  };

  let explode_bounds_component = ExplodeBoundsComponent;

  LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(rigid_body_component)
  .with(garbage_component)
  .with(bounds_component)
  .with(explode_bounds_component)
  .build()
}

pub fn create_out_of_bounds_explosion(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  position_component: PositionComponent,
) -> Entity {
  let line_width = 20.0;
  let rect = graphics::Rect::new(-line_width / 2.0, -line_width / 2.0, line_width, line_width);
  let mesh = graphics::MeshBuilder::new()
    .polygon(graphics::DrawMode::Fill, &rect_to_polygon(rect))
    .clone();

  let draw_param = graphics::DrawParam {
    color: Some(HP_COLOR()),
    ..Default::default()
  };

  let mesh_component = MeshComponent { mesh, draw_param };

  let lifetime_component = LifetimeComponent {
    duration: Duration::from_millis(250),
  };;

  let garbage_component = GarbageComponent::default();

  LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(lifetime_component)
  .with(garbage_component)
  .build()
}

pub fn create_death_explosion(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  physics_world: &mut PhysicsWorld,
  x: f32,
  y: f32,
) -> Entity {
  let mut rng = rand::thread_rng();

  let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
  let length = rng.gen_range(10.0, 50.0);
  let velocity = rng.gen_range(75.0, 150.0);
  let line_width = 2.0;

  let position_component = PositionComponent { x, y, angle };

  let rigid_body = RigidBodyDesc::new()
    .position(Isometry2::new(Vector2::new(x, y), angle))
    .status(BodyStatus::Dynamic)
    .build(physics_world);

  rigid_body.set_linear_velocity(Vector2::new(angle.cos() * velocity, angle.sin() * velocity));

  let rigid_body_component = RigidBodyComponent {
    handle: rigid_body.handle(),
  };

  let mesh = graphics::MeshBuilder::new()
    .line(
      &[
        graphics::Point2::origin(),
        graphics::Point2::new(length, 0.0),
      ],
      line_width,
    )
    .clone();

  let draw_param = graphics::DrawParam {
    color: Some(HP_COLOR()),
    ..Default::default()
  };

  let mesh_component = MeshComponent { mesh, draw_param };

  let lifetime_component = LifetimeComponent {
    duration: Duration::from_millis(4000),
  };

  let garbage_component = GarbageComponent::default();

  LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(rigid_body_component)
  .with(mesh_component)
  .with(lifetime_component)
  .with(garbage_component)
  .build()
}
