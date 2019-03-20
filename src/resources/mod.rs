use nphysics2d::world::World;

#[derive(Debug, Default)]
pub struct UpdateTime(pub f32);

pub type PhysicsWorld = World<f32>;

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
    Input::new()
  }
}
