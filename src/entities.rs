use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, ConvexPolygon, Cuboid, ShapeHandle};
use nphysics2d::math::{Inertia, Velocity};
use nphysics2d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBody, RigidBodyDesc};
use uuid::Uuid;

use crate::resources::*;
use crate::*;

#[derive(Debug, Clone)]
pub enum GameAction {
  Nothing,
  CreateProjectile { projectile: Projectile },
  CreateEffect { effect: Effect },
  DestroyEffect { id: Uuid },
  DestroyProjectile { id: Uuid },
}

pub struct Input {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub attack: bool,
}

impl Input {
  pub fn new() -> Input {
    Input {
      up: false,
      down: false,
      left: false,
      right: false,
      attack: false,
    }
  }
}

impl Default for Input {
  fn default() -> Input {
    Input {
      up: false,
      down: false,
      left: false,
      right: false,
      attack: false,
    }
  }
}

#[derive(Debug, Clone)]
pub enum EffectType {
  Shooting { color: graphics::Color },
}

#[derive(Debug, Clone)]
pub struct Effect {
  pub id: Uuid,
  pub x: f32,
  pub y: f32,
  pub lifetime: f32, // seconds
  pub age: f32,      // seconds
  pub effect_type: EffectType,
}

impl Effect {
  pub fn new(x: f32, y: f32, lifetime: f32, effect_type: EffectType) -> Effect {
    Effect {
      id: Uuid::new_v4(),
      x,
      y,
      lifetime,
      age: 0.0,
      effect_type,
    }
  }

  pub fn is_dead(&self) -> bool {
    self.age >= self.lifetime
  }

  pub fn update(&mut self, _ctx: &mut Context, dt_seconds: f32) -> Vec<GameAction> {
    let mut actions: Vec<GameAction> = Vec::new();

    self.age += dt_seconds;

    if self.age >= self.lifetime {
      actions.push(GameAction::DestroyEffect { id: self.id });
    }

    actions
  }
  pub fn draw(&self, ctx: &mut Context) {
    let point = graphics::Point2::new(self.x, self.y);

    match self.effect_type {
      EffectType::Shooting { color } => {
        let old_color = graphics::get_color(ctx);
        graphics::set_color(ctx, color).unwrap();
        graphics::circle(ctx, graphics::DrawMode::Fill, point, 2.0, 0.1).unwrap();
        graphics::set_color(ctx, old_color).unwrap();
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum ProjectileType {
  Basic { radius: f32 },
}

#[derive(Debug, Clone)]
pub struct Projectile {
  pub id: Uuid,
  pub rigid_body_handle: BodyHandle,
  pub x: f32,
  pub y: f32,
  pub velocity: f32,
  pub angle: f32,
  pub projectile_type: ProjectileType,
}

impl Projectile {
  pub fn new(
    physics_world: &mut PhysicsWorld,
    x: f32,
    y: f32,
    velocity: f32,
    angle: f32,
    projectile_type: ProjectileType,
  ) -> Projectile {
    let radius = 10.0;

    let rigid_body_handle = RigidBodyDesc::new()
      .collider(&ColliderDesc::new(ShapeHandle::new(Ball::new(radius))))
      .position(Isometry2::new(Vector2::new(x, y), angle))
      .status(BodyStatus::Dynamic)
      .gravity_enabled(false)
      .build(physics_world)
      .handle();

    Projectile {
      id: Uuid::new_v4(),
      rigid_body_handle,
      x,
      y,
      velocity,
      angle,
      projectile_type,
    }
  }

  pub fn update(
    &mut self,
    _ctx: &mut Context,
    physics_world: &mut PhysicsWorld,
  ) -> Vec<GameAction> {
    let mut actions: Vec<GameAction> = Vec::new();

    let body: &mut RigidBody<f32> = physics_world
      .rigid_body_mut(self.rigid_body_handle)
      .expect("Rigid body in specs does not exist in physics world");

    self.x = body.position().translation.x;
    self.y = body.position().translation.y;

    body.set_linear_velocity(Vector2::new(
      self.angle.cos() * self.velocity,
      self.angle.sin() * self.velocity,
    ));

    actions
  }
  pub fn draw(&self, ctx: &mut Context) {
    let point = graphics::Point2::new(self.x, self.y);

    match self.projectile_type {
      ProjectileType::Basic { radius } => {
        graphics::circle(ctx, graphics::DrawMode::Line(1.0), point, radius, 0.1).unwrap();
      }
    }
  }
}

pub struct Player {
  rigid_body_handle: BodyHandle,
  x: f32,
  y: f32,
  velocity: f32,
  max_velocity: f32,
  acceleration: f32,
  angle: f32,
  radius: f32,
  thickness: f32,
}

impl Player {
  pub fn new(physics_world: &mut PhysicsWorld, width: u32, height: u32) -> Player {
    let x = width as f32 / 2.0;
    let y = height as f32 / 2.0;
    let radius = 10.0;
    let angle = std::f32::consts::PI / 2.0;
    let velocity = 0.0;
    let max_velocity = 200.0;
    let acceleration = 100.0;
    let thickness = 2.0;

    let rigid_body_handle = RigidBodyDesc::new()
      .collider(&ColliderDesc::new(ShapeHandle::new(Ball::new(radius))))
      .position(Isometry2::new(Vector2::new(x, y), angle))
      .status(BodyStatus::Dynamic)
      .gravity_enabled(false)
      .build(physics_world)
      .handle();

    Player {
      rigid_body_handle,
      x,
      y,
      velocity,
      max_velocity,
      acceleration,
      angle,
      radius,
      thickness,
    }
  }

  pub fn update(
    &mut self,
    _ctx: &mut Context,
    dt_seconds: f32,
    physics_world: &mut PhysicsWorld,
    input: &Input,
  ) -> Vec<GameAction> {
    let mut actions: Vec<GameAction> = Vec::new();

    {
      let body: &mut RigidBody<f32> = physics_world
        .rigid_body_mut(self.rigid_body_handle)
        .expect("Rigid body in specs does not exist in physics world");

      if input.left {
        self.angle -= 0.05;
      } else if input.right {
        self.angle += 0.05;
      };

      self.velocity = f32::min(self.velocity + self.acceleration * 0.1, self.max_velocity);

      body.set_linear_velocity(Vector2::new(
        self.angle.cos() * self.velocity,
        self.angle.sin() * self.velocity,
      ));

      self.x = body.position().translation.x;
      self.y = body.position().translation.y;
    }

    {
      if input.attack {
        actions.extend(self.shoot(physics_world));
      }
    }

    actions
  }

  pub fn draw(&self, ctx: &mut Context) {
    let point = graphics::Point2::new(self.x, self.y);

    graphics::circle(ctx, graphics::DrawMode::Line(1.0), point, self.radius, 0.1).unwrap();
    graphics::line(
      ctx,
      &[
        point,
        graphics::Point2::new(
          self.x + 2.0 * self.radius * self.angle.cos(),
          self.y + 2.0 * self.radius * self.angle.sin(),
        ),
      ],
      self.thickness,
    )
    .unwrap();
  }

  fn shoot(&self, physics_world: &mut PhysicsWorld) -> Vec<GameAction> {
    let create_projectile_action = GameAction::CreateProjectile {
      projectile: Projectile::new(
        physics_world,
        self.x,
        self.y,
        400.0,
        self.angle,
        ProjectileType::Basic { radius: 2.0 },
      ),
    };

    let create_shoot_effect = GameAction::CreateEffect {
      effect: Effect::new(
        self.x,
        self.y,
        0.24,
        EffectType::Shooting {
          color: graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        },
      ),
    };

    vec![create_projectile_action, create_shoot_effect]
  }
}
