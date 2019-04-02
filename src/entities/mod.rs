use crate::components::*;
use crate::resources::*;
use crate::utils::*;

use core::ops::{Deref, DerefMut};
use ggez::graphics;
use ggez::Context;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionGroups;
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
  physics: &mut PhysicsSim,
) -> () {
  let x = game_settings.width as f32 / 2.0;
  let y = game_settings.height as f32 / 2.0;
  let velocity = 0.0;
  let angle = 0.5;
  let size = 25.0;

  let collision_groups = CollisionGroups::new()
    .with_membership(&[PLAYER_BODY_COLLISION_GROUP])
    .with_blacklist(&[PLAYER_PROJECTILE_COLLISION_GROUP]);

  let collider_desc = ColliderDesc::new(ShapeHandle::new(Ball::new(size)))
    .collision_groups(collision_groups)
    .name(ColliderType::Player.to_string());;

  collider_desc.build(&mut physics.world);

  let rigid_body_handle = RigidBodyDesc::new()
    .collider(&collider_desc)
    .position(Isometry2::new(Vector2::new(x, y), angle))
    .velocity(Velocity::linear(velocity, 0.0))
    .status(BodyStatus::Dynamic)
    .build(&mut physics.world)
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

  let entity = LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(rigid_body_component)
  .with(controllable_component)
  .with(shooting_component)
  .with(garbage_component)
  .build();

  physics.bodies.insert(rigid_body_handle, entity);
}

pub fn create_projectile(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  settings: &GameSettings,
  physics: &mut PhysicsSim,
  position_component: PositionComponent,
) -> () {
  let x = position_component.x;
  let y = position_component.y;
  let angle = position_component.angle;
  let velocity = 500.0;

  let radius = 4.0;
  let tolerance = 0.1;
  let thickness = 1.0;

  let collision_groups = CollisionGroups::new()
    .with_membership(&[PLAYER_PROJECTILE_COLLISION_GROUP])
    .with_blacklist(&[PLAYER_BODY_COLLISION_GROUP]);

  let collider_desc = ColliderDesc::new(ShapeHandle::new(Ball::new(radius)))
    .collision_groups(collision_groups)
    .name(ColliderType::PlayerProjectile.to_string());;

  collider_desc.build(&mut physics.world);

  let rigid_body = RigidBodyDesc::new()
    .collider(&collider_desc)
    .position(Isometry2::new(Vector2::new(x, y), angle))
    .status(BodyStatus::Dynamic)
    .build(&mut physics.world);

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
    x_min: 0.0,
    x_max: settings.width as f32,
    y_min: 0.0,
    y_max: settings.height as f32,
  };

  let explode_bounds_component = ExplodeBoundsComponent;

  let entity = LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(rigid_body_component)
  .with(garbage_component)
  .with(bounds_component)
  .with(explode_bounds_component)
  .build();

  physics.bodies.insert(rigid_body.handle(), entity);
}

pub fn create_out_of_bounds_explosion(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  position_component: PositionComponent,
) -> () {
  let line_width = 20.0;
  let rect = graphics::Rect::new(-line_width / 2.0, -line_width / 2.0, line_width, line_width);
  let mesh = graphics::MeshBuilder::new()
    .polygon(graphics::DrawMode::Fill, &rect_to_polygon(rect))
    .clone();

  let draw_param = graphics::DrawParam {
    color: Some(hp_color()),
    ..Default::default()
  };

  let mesh_component = MeshComponent { mesh, draw_param };

  let lifetime_component = LifetimeComponent {
    duration: Duration::from_millis(250),
  };

  let garbage_component = GarbageComponent::default();

  LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(lifetime_component)
  .with(garbage_component)
  .build();
}

pub fn create_death_explosion(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  physics: &mut PhysicsSim,
  x: f32,
  y: f32,
) -> () {
  let mut rng = rand::thread_rng();

  let count = rng.gen_range(4, 30);

  for _ in 0..count {
    let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    let length = rng.gen_range(15.0, 30.0);
    let velocity = rng.gen_range(100.0, 300.0);
    let line_width = 2.0;

    let position_component = PositionComponent { x, y, angle };

    let rigid_body = RigidBodyDesc::new()
      .position(Isometry2::new(Vector2::new(x, y), angle))
      .status(BodyStatus::Dynamic)
      .build(&mut physics.world);

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
      color: Some(hp_color()),
      ..Default::default()
    };

    let mesh_component = MeshComponent { mesh, draw_param };

    let tween_component = TweenComponent {
      tween_type: Tween::SizeTween {
        ease: Ease::Cubic,
        starting: 1.0,
        ending: 0.0,
      },
      elapsed: Duration::from_millis(0),
      duration: Duration::from_millis(1000),
    };

    let lifetime_component = LifetimeComponent {
      duration: Duration::from_millis(1000),
    };

    let garbage_component = GarbageComponent::default();

    let entity = LazyBuilder {
      entity: entities.create(),
      lazy: lazy,
    }
    .with(position_component)
    .with(rigid_body_component)
    .with(mesh_component)
    .with(tween_component)
    .with(lifetime_component)
    .with(garbage_component)
    .build();
  }
}

pub fn create_ammo(
  entities: &EntitiesRes,
  lazy: &LazyUpdate,
  settings: &GameSettings,
  physics: &mut PhysicsSim,
) -> () {
  let mut rng = rand::thread_rng();

  let offset = 48.0;
  let direction = if rand::random() { -1.0 } else { 1.0 };
  let x = settings.width as f32 / 2.0 + direction * (settings.width as f32 / 2.0 + offset);
  let y = rng.gen_range(offset, settings.height as f32 - offset);

  let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
  let velocity = -direction * rng.gen_range(30.0, 100.0);
  let radius = 15.0;
  let thickness = 3.0;

  let position_component = PositionComponent { x, y, angle };

  let collision_groups = CollisionGroups::new().with_membership(&[AMMO_BODY_COLLISION_GROUP]);

  let collider_desc = ColliderDesc::new(ShapeHandle::new(Ball::new(radius)))
    .collision_groups(collision_groups)
    .name(ColliderType::Ammo.to_string());

  collider_desc.build(&mut physics.world);

  let rigid_body = RigidBodyDesc::new()
    .collider(&collider_desc)
    .position(Isometry2::new(Vector2::new(x, y), angle))
    .status(BodyStatus::Dynamic)
    .build(&mut physics.world);

  rigid_body.set_linear_velocity(Vector2::new(velocity, 0.0));
  rigid_body.set_angular_velocity(2.0);

  let rigid_body_component = RigidBodyComponent {
    handle: rigid_body.handle(),
  };

  let rect = graphics::Rect::new(-radius, -radius, radius, radius);
  let mesh = graphics::MeshBuilder::new()
    .polygon(graphics::DrawMode::Line(thickness), &rect_to_polygon(rect))
    .clone();

  let draw_param = graphics::DrawParam {
    color: Some(ammo_color()),
    ..Default::default()
  };

  let mesh_component = MeshComponent { mesh, draw_param };

  let garbage_component = GarbageComponent::default();

  let bounds_component = BoundsComponent {
    x_min: 0.0 - offset,
    x_max: settings.width as f32 + offset,
    y_min: 0.0,
    y_max: settings.height as f32,
  };

  let entity = LazyBuilder {
    entity: entities.create(),
    lazy: lazy,
  }
  .with(position_component)
  .with(mesh_component)
  .with(rigid_body_component)
  .with(garbage_component)
  .with(bounds_component)
  .build();

  physics.bodies.insert(rigid_body.handle(), entity);
}
