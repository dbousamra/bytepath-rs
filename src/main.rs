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

mod entities;
mod resources;

use entities::*;
use resources::*;

#[macro_use]
extern crate specs_derive;

struct MainState {
  physics_world: PhysicsWorld,
  width: u32,
  height: u32,
  input: Input,
  player: Player,
  projectiles: Vec<Projectile>,
  effects: Vec<Effect>,
}

impl MainState {
  fn new(_ctx: &mut Context, width: u32, height: u32) -> GameResult<MainState> {
    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(Vector2::new(0.0, 0.0));
    physics_world.set_contact_model(SignoriniModel::new());

    let input = Input::default();

    let player = Player::new(&mut physics_world, width, height);

    let projectiles = Vec::new();
    let effects = Vec::new();

    Ok(MainState {
      physics_world,
      width,
      height,
      input,
      player,
      projectiles,
      effects,
    })
  }

  fn process_action(&mut self, _ctx: &mut Context, action: GameAction) {
    match action {
      GameAction::CreateProjectile { projectile } => self.projectiles.push(projectile),
      GameAction::CreateEffect { effect } => self.effects.push(effect),
      GameAction::DestroyEffect { id } => self.effects.retain(|effect| id != effect.id),
      GameAction::DestroyProjectile { id } => {
        for projectile in self.projectiles.iter() {
          if projectile.id == id {
            self
              .physics_world
              .remove_bodies(&[projectile.rigid_body_handle]);
          }
        }

        self.projectiles.retain(|projectile| id != projectile.id)
      }
      GameAction::Nothing => (),
    }
  }

  fn check_bounds(&self) -> Vec<GameAction> {
    let mut actions: Vec<GameAction> = Vec::new();

    for projectile in self.projectiles.iter() {
      let out_of_x_bounds = projectile.x < 0.0 || projectile.x > self.width as f32;
      let out_of_y_bounds = projectile.y < 0.0 || projectile.y > self.height as f32;

      if out_of_x_bounds || out_of_y_bounds {
        actions.push(GameAction::DestroyProjectile { id: projectile.id })
      }
    }

    actions
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dt = timer::get_delta(ctx);
    let dt_seconds = dt.subsec_nanos() as f32 / 1_000_000_000.0;

    if timer::get_ticks(ctx) % 100 == 0 {
      println!("Average FPS: {}", timer::get_fps(ctx));
    }

    let mut actions: Vec<GameAction> = Vec::new();

    actions.extend(
      self
        .player
        .update(ctx, dt_seconds, &mut self.physics_world, &self.input),
    );

    for projectile in self.projectiles.iter_mut() {
      actions.extend(projectile.update(ctx, &mut self.physics_world))
    }

    for effect in self.effects.iter_mut() {
      actions.extend(effect.update(ctx, dt_seconds))
    }

    actions.extend(self.check_bounds());

    for action in actions {
      self.process_action(ctx, action);
    }
    self.physics_world.set_timestep(dt_seconds);
    self.physics_world.step();

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    self.player.draw(ctx);

    for projectile in self.projectiles.iter_mut() {
      projectile.draw(ctx);
    }

    for effect in self.effects.iter_mut() {
      effect.draw(ctx);
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
    if !repeat {
      match keycode {
        Keycode::Left => self.input.left = false,
        Keycode::Right => self.input.right = false,
        Keycode::Up => self.input.up = false,
        Keycode::Down => self.input.down = false,
        Keycode::Space => self.input.attack = false,
        _ => (),
      }
    }
  }

  fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
    if !repeat {
      match keycode {
        Keycode::Left => self.input.left = true,
        Keycode::Right => self.input.right = true,
        Keycode::Up => self.input.up = true,
        Keycode::Down => self.input.down = true,
        Keycode::Space => self.input.attack = true,
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

  let cb = ggez::ContextBuilder::new("bytepath", "ggez").window_mode(window_mode);
  let ctx = &mut cb.build().unwrap();

  match MainState::new(ctx, width, height) {
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
