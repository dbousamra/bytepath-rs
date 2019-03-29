use specs::{Component, VecStorage};

use std::time::Duration;

#[derive(Debug, Clone)]

pub enum Ease {
  Cubic,
}

#[derive(Debug, Clone)]
pub enum Tween {
  SizeTween {
    ease: Ease,
    starting: f32, // percentage
    ending: f32,
  },
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct TweenComponent {
  pub tween_type: Tween,
  pub elapsed: Duration,
  pub duration: Duration,
}
