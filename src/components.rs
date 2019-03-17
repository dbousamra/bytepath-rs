extern crate ggez;
use ggez::graphics::{DrawMode, Point2};
use nalgebra::{Isometry2, Vector2};
use nphysics2d::object::{BodyHandle, BodyStatus, RigidBody};
use specs::prelude::*;

#[derive(Debug, Component, Clone)]
#[storage(VecStorage)]
pub struct PositionComponent {
  pub x: f32,
  pub y: f32,
  pub angle: f32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct RigidBodyComponent {
  pub handle: BodyHandle,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct BoundsComponent {
  pub x_min: f32,
  pub x_max: f32,
  pub y_min: f32,
  pub y_max: f32,
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
pub struct ControllableComponent;

#[derive(Debug, Clone)]
pub enum Shape {
  Circle(Vector2<f32>, f32, DrawMode),
  Line(Vector2<f32>, f32, f32),
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ShapesComponent {
  pub shapes: Vec<Shape>,
}
