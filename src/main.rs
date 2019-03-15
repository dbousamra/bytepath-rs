extern crate ggez;

use ggez::event::*;
use ggez::*;
use ggez::{Context, GameResult};

use specs::prelude::*;
use specs::{Dispatcher, DispatcherBuilder, RunNow, World};

mod components;
mod systems;

use components::*;
use systems::*;

#[macro_use]
extern crate specs_derive;

struct Systems {
  movement: MovementSystem,
  controllable: ControllableSystem,
}

struct MainState {
  world: World,
  systems: Systems,
}

impl MainState {
  fn new(_ctx: &mut Context, world: World, systems: Systems) -> GameResult<MainState> {
    Ok(MainState { world, systems })
  }
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    if timer::get_ticks(ctx) % 100 == 0 {
      let dt = timer::get_delta(ctx);
      let seconds = dt.subsec_nanos() as f32 / 1_000_000_000.0;
      println!("Delta Time (seconds): {:?} ", seconds);
      println!("Average FPS: {}", timer::get_fps(ctx));
    }
    self.systems.movement.run_now(&self.world.res);
    self.systems.controllable.run_now(&self.world.res);
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    {
      let mut rs = RenderingSystem { ctx };
      rs.run_now(&self.world.res);
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
    let mut input = self.world.write_resource::<Input>();
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
    let mut input = self.world.write_resource::<Input>();
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

  let mut world = World::new();
  world.add_resource(Input::new());
  world.register::<PositionComponent>();
  world.register::<VelocityComponent>();
  world.register::<ShapeComponent>();
  world.register::<ControllableComponent>();

  // let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
  //   .with(StatesSystem, "states", &[])
  //   .with(ControlableSystem, "controlable", &[])
  //   .with(PhysicSystem, "physic_system", &[])
  //   .with(MoveSystem, "move", &["physic_system"])
  //   .with(ChaseCameraSystem, "chase_camera", &["move"])
  //   .with(SnapCameraSystem, "snap_camera", &["move"])
  //   .build();

  world
    .create_entity()
    .with(ControllableComponent {})
    .with(PositionComponent {
      x: (WIDTH / 2) as f32,
      y: (HEIGHT / 2) as f32,
    })
    .with(VelocityComponent {
      x: 0.0,
      y: 0.0,
      r: 0.0,
    })
    .with(ShapeComponent {
      variant: Shape::Circle(10.0, ggez::graphics::DrawMode::Line(1.0)),
    })
    .build();

  let systems = Systems {
    controllable: ControllableSystem,
    movement: MovementSystem {
      width: WIDTH,
      height: HEIGHT,
    },
  };

  let state = &mut MainState::new(ctx, world, systems).unwrap();

  event::run(ctx, state).unwrap();
}
