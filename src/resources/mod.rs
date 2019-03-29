use nphysics2d::world::World;
use shrev::EventChannel;
use std::fmt::{Display, Formatter};
use std::result::Result;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct UpdateTime(pub Duration);

pub type PhysicsWorld = World<f32>;

pub type CollisionEvents = EventChannel<CollisionEvent>;

#[derive(Debug)]
pub struct CollisionEvent {
  pub collider1: ColliderType,
  pub collider2: ColliderType,
  pub x: f32,
  pub y: f32,
}

pub const PLAYER_BODY_COLLISION_GROUP: usize = 0;
pub const PLAYER_PROJECTILE_COLLISION_GROUP: usize = 1;
pub const AMMO_BODY_COLLISION_GROUP: usize = 2;

#[derive(Debug)]
pub enum ColliderType {
  Player,
  PlayerProjectile,
  Ammo,
}

impl FromStr for ColliderType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Player" => Ok(ColliderType::Player),
      "PlayerProjectile" => Ok(ColliderType::PlayerProjectile),
      "Ammo" => Ok(ColliderType::Ammo),
      _ => Err(format!("Unable to parse {} as ColliderType", s)),
    }
  }
}

impl Display for ColliderType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct Input {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub attack: bool,
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

pub struct GameSettings {
  pub width: u32,
  pub height: u32,
  pub scale: u32,
}

impl Default for GameSettings {
  fn default() -> GameSettings {
    GameSettings {
      width: 1920,
      height: 1080,
      scale: 1,
    }
  }
}

pub struct SpawnInfo {
  pub ammo_last: Instant,
  pub ammo_every: Duration,
}

impl Default for SpawnInfo {
  fn default() -> SpawnInfo {
    SpawnInfo {
      ammo_last: Instant::now(),
      ammo_every: Duration::from_millis(30),
    }
  }
}
