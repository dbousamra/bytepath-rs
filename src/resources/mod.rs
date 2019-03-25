use nphysics2d::world::World;

use std::time::Duration;

#[derive(Debug, Default)]
pub struct UpdateTime(pub Duration);

pub type PhysicsWorld = World<f32>;

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
      scale: 4,
    }
  }
}
