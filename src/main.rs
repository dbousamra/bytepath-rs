#![allow(dead_code)]
#![allow(unused_imports)]

extern crate ggez;
extern crate nalgebra;

use ggez::event::*;
use ggez::*;
use ggez::{Context, GameResult};

use nalgebra::Vector2;
use nphysics2d::solver::SignoriniModel;
use specs::prelude::*;
use specs::{RunNow, World};

mod components;
mod entities;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

#[macro_use]
extern crate specs_derive;

struct MainState<'a, 'b> {
  specs_world: World,
  dispatcher: Dispatcher<'a, 'b>,
  width: u32,
  height: u32,
}

impl<'a, 'b> MainState<'a, 'b> {
  fn new(ctx: &mut Context, width: u32, height: u32) -> GameResult<MainState<'a, 'b>> {
    let mut specs_world = World::new();

    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(Vector2::new(0.0, 0.0));
    physics_world.set_contact_model(SignoriniModel::new());

    specs_world.add_resource(physics_world);
    specs_world.add_resource(Input::new());
    specs_world.add_resource(UpdateTime(0.0));

    specs_world.register::<ControllableComponent>();
    specs_world.register::<MeshComponent>();
    specs_world.register::<PositionComponent>();
    specs_world.register::<RigidBodyComponent>();

    let dispatcher = DispatcherBuilder::new()
      .with(PhysicsSystem, "physics_system", &[])
      .with(PositionSystem, "position_system", &["physics_system"])
      .build();

    entities::create_player(ctx, &mut specs_world);

    Ok(MainState {
      specs_world,
      dispatcher,
      width,
      height,
    })
  }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dt = timer::get_delta(ctx);
    let dt_seconds = dt.subsec_nanos() as f32 / 1_000_000_000.0;

    if timer::get_ticks(ctx) % 100 == 0 {
      println!("Average FPS: {}", timer::get_fps(ctx));
    }

    self.specs_world.write_resource::<UpdateTime>().0 = dt_seconds;
    self.specs_world.write_resource::<PhysicsWorld>();
    self.specs_world.write_resource::<ControllableSystem>();
    self.dispatcher.dispatch(&mut self.specs_world.res);
    self.specs_world.maintain();

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    {
      let mut rs = RenderingSystem { ctx };
      rs.run_now(&self.specs_world.res);
    }

    graphics::present(ctx);
    Ok(())
  }

  fn key_up_event(
    &mut self,
    _ctx: &mut Context,
    keycode: ggez::event::Keycode,
    _keymod: Mod,
    repeat: bool,
  ) {
    let mut input = self.specs_world.write_resource::<Input>();

    if !repeat {
      match keycode {
        Keycode::Left => input.left = false,
        Keycode::Right => input.right = false,
        Keycode::Up => input.up = false,
        Keycode::Down => input.down = false,
        Keycode::Space => input.attack = false,
        _ => (),
      }
    }
  }

  fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
    let mut input = self.specs_world.write_resource::<Input>();

    if !repeat {
      match keycode {
        Keycode::Left => input.left = true,
        Keycode::Right => input.right = true,
        Keycode::Up => input.up = true,
        Keycode::Down => input.down = true,
        Keycode::Space => input.attack = true,
        _ => (),
      }
    }
  }
}

fn main() {
  let width = 800;
  let height = 600;

  let window_mode = ggez::conf::WindowMode {
    width: width,
    height: height,
    borderless: false,
    fullscreen_type: ggez::conf::FullscreenType::Off,
    vsync: true,
    min_width: 0,
    min_height: 0,
    max_width: 0,
    max_height: 0,
  };

  let ctx = &mut ggez::ContextBuilder::new("bytepath", "ggez")
    .window_mode(window_mode)
    .build()
    .unwrap();

  let mut state = MainState::new(ctx, width, height).unwrap();
  event::run(ctx, &mut state).unwrap();
}
