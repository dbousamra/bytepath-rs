use nphysics2d::world::World;

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
