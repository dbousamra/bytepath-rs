#![allow(dead_code)]
#![allow(unused_imports)]

extern crate ggez;
extern crate nalgebra;
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::*;
use ggez::graphics::Canvas;
use ggez::*;
use ggez::{Context, GameResult};
use nalgebra::Vector2;
use nphysics2d::solver::SignoriniModel;
use shrev::EventChannel;
use specs::prelude::*;
use specs::{RunNow, World};

mod components;
mod entities;
mod resources;
mod systems;
mod utils;

use components::*;
use resources::*;
use systems::*;
use utils::*;

#[macro_use]
extern crate specs_derive;

struct MainState<'a, 'b> {
  specs_world: World,
  dispatcher: Dispatcher<'a, 'b>,
  canvas: Canvas,
}

impl<'a, 'b> MainState<'a, 'b> {
  fn new(ctx: &mut Context, game_settings: GameSettings) -> GameResult<MainState<'a, 'b>> {
    let canvas = Canvas::new(
      ctx,
      game_settings.width / game_settings.scale,
      game_settings.height / game_settings.scale,
      NumSamples::One,
    )
    .unwrap();

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    let mut specs_world = World::new();

    // let mut physics_world = PhysicsWorld::new();
    // physics_world.set_gravity(Vector2::new(0.0, 0.0));
    // physics_world.set_contact_model(SignoriniModel::new());

    // specs_world.add_resource(UpdateTime::default());
    // specs_world.add_resource(Input::default());
    // specs_world.add_resource(SpawnInfo::default());
    // specs_world.add_resource(physics_world);
    // specs_world.add_resource(EventChannel::<bool>::new());
    // specs_world.add_resource(game_settings);

    let mut dispatcher = DispatcherBuilder::new()
      .with(PhysicsSystem, "physics_system", &[])
      .with(
        CollisionSystem::default(),
        "collision_system",
        &["physics_system"],
      )
      .with(PositionSystem, "position_system", &["physics_system"])
      .with(
        ControllableSystem,
        "controllable_system",
        &["physics_system"],
      )
      .with(ShootingSystem, "shooting_system", &[])
      .with(BoundsSystem, "bounds_system", &["position_system"])
      .with(GarbageSystem, "garbage_system", &[])
      .with(LifetimeSystem, "lifetime_system", &[])
      .with(TweenSystem, "tween_system", &[])
      .with(SpawnSystem, "spawn_system", &[])
      .build();

    dispatcher.setup(&mut specs_world.res);

    // I don't know what I'm doing. Is this needed?
    {
      let lazy = specs_world.read_resource::<LazyUpdate>();
      let mut physics_world = specs_world.write_resource::<PhysicsWorld>();
      let game_settings = specs_world.read_resource::<GameSettings>();
      entities::create_player(
        ctx,
        &specs_world.entities(),
        &lazy,
        &game_settings,
        &mut physics_world,
      );
    }

    Ok(MainState {
      specs_world,
      dispatcher,
      canvas,
    })
  }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dt = timer::get_delta(ctx);

    if timer::get_ticks(ctx) % 100 == 0 {
      println!("Average FPS: {}", timer::get_fps(ctx));
    }

    self.specs_world.write_resource::<UpdateTime>().0 = dt;
    self.specs_world.write_resource::<PhysicsWorld>();
    self.dispatcher.dispatch(&mut self.specs_world.res);
    self.specs_world.maintain();

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let game_settings = self.specs_world.read_resource::<GameSettings>();

    graphics::set_canvas(ctx, Some(&self.canvas));
    graphics::set_background_color(ctx, utils::backround_color());
    graphics::clear(ctx);

    {
      let mut rs = RenderingSystem { ctx };
      rs.run_now(&self.specs_world.res);
    }
    graphics::set_canvas(ctx, None);

    graphics::draw_ex(
      ctx,
      &self.canvas,
      graphics::DrawParam {
        scale: graphics::Point2::new(game_settings.scale as f32, game_settings.scale as f32),
        ..Default::default()
      },
    )
    .unwrap();

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

  fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
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

    if keycode == Keycode::Q {
      ctx.quit().expect("Should never fail");
    }
  }
}

fn main() {
  let game_settings = GameSettings {
    width: 1920,
    height: 1080,
    scale: 1,
  };

  let window_mode = WindowMode {
    width: game_settings.width,
    height: game_settings.height,
    borderless: false,
    fullscreen_type: ggez::conf::FullscreenType::Off,
    vsync: true,
    min_width: 0,
    min_height: 0,
    max_width: 0,
    max_height: 0,
  };

  let window_setup = WindowSetup {
    title: "BYTEPATH-rs".to_owned(),
    allow_highdpi: false,
    ..Default::default()
  };

  let ctx = &mut ggez::ContextBuilder::new("bytepath", "ggez")
    .window_mode(window_mode)
    .window_setup(window_setup)
    .build()
    .unwrap();

  let mut state = MainState::new(ctx, game_settings).unwrap();
  event::run(ctx, &mut state).unwrap();
}
