#![allow(dead_code)]
#![allow(unused_imports)]

extern crate ggez;
extern crate nalgebra;

use ggez::event::*;
use ggez::*;
use ggez::{Context, GameResult};

use nalgebra::Vector2;
use specs::prelude::*;
use specs::{RunNow, World};

mod components;
mod entities;
mod resources;
mod systems;

use components::*;
use entities::*;
use resources::*;
use systems::*;

#[macro_use]
extern crate specs_derive;

struct Systems {
  physics: PhysicsSystem,
  controllable: ControllableSystem,
}

struct MainState<'a, 'b> {
  specs_world: World,
  dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
  fn new(_ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
    let mut specs_world = World::new();
    specs_world.add_resource(Input::new());
    specs_world.register::<PositionComponent>();
    specs_world.register::<ShapesComponent>();
    specs_world.register::<RigidBodyComponent>();
    specs_world.register::<ControllableComponent>();
    specs_world.register::<BoundsComponent>();

    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(Vector2::new(0.0, 0.0));
    specs_world.add_resource(physics_world);
    specs_world.add_resource(UpdateTime(0.0));

    create_player(_ctx, &mut specs_world, WIDTH as f32, HEIGHT as f32);

    let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
      .with(ControllableSystem, "controlable", &[])
      .with(PhysicsSystem, "physics_system", &[])
      .with(PositionSystem, "position_system", &["physics_system"])
      .with(BoundsSystem, "bounds_system", &["physics_system"])
      .build();

    Ok(MainState {
      specs_world: specs_world,
      dispatcher: dispatcher,
    })
  }
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dt = timer::get_delta(ctx);
    let seconds = dt.subsec_nanos() as f32 / 1_000_000_000.0;

    if timer::get_ticks(ctx) % 100 == 0 {
      println!("Average FPS: {}", timer::get_fps(ctx));
    }

    self.specs_world.write_resource::<UpdateTime>().0 = seconds;
    self.specs_world.write_resource::<PhysicsWorld>();

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
        Keycode::LCtrl => input.slide = true,
        Keycode::Space => input.jump = true,
        Keycode::LShift => input.attack = true,
        _ => (),
      }
    }
  }
}

fn main() {
  let window_mode = ggez::conf::WindowMode {
    width: WIDTH,
    height: HEIGHT,
    borderless: false,
    fullscreen_type: ggez::conf::FullscreenType::Off,
    vsync: true,
    min_width: 0,
    min_height: 0,
    max_width: 0,
    max_height: 0,
  };

  let cb = ggez::ContextBuilder::new("bytepath", "ggez").window_mode(window_mode);
  let ctx = &mut cb.build().unwrap();

  match MainState::new(ctx) {
    Ok(ref mut game) => {
      let result = event::run(ctx, game);
      if let Err(e) = result {
        println!("Error encountered running game: {}", e);
      } else {
        println!("Game exited cleanly.");
      }
    }
    Err(e) => {
      println!("Could not load game!");
      println!("Error: {}", e);
    }
  }
}
