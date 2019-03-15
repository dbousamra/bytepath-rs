extern crate ggez;
use ggez::graphics::{DrawMode, Point2};
use specs::prelude::*;

#[derive(Debug, Component, Clone)]
#[storage(VecStorage)]
pub struct PositionComponent {
  pub x: f32,
  pub y: f32,
}

impl PositionComponent {
  pub fn to_point(&self) -> Point2 {
    Point2::new(self.x, self.y)
  }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct VelocityComponent {
  pub x: f32,
  pub y: f32,
  pub r: f32,
}

#[derive(Debug, Clone)]
pub enum Shape {
  Circle(f32, DrawMode),
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ShapeComponent {
  pub variant: Shape,
}

#[derive(Clone, Debug)]
pub struct Input {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub slide: bool,
  pub jump: bool,
  pub attack: bool,
}

impl Input {
  pub fn new() -> Input {
    Input {
      up: false,
      down: false,
      left: false,
      right: false,
      slide: false,
      jump: false,
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
      slide: false,
      jump: false,
      attack: false,
    }
  }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ControllableComponent {}
