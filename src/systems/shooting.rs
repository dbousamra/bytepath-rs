use crate::components::*;
use crate::entities::*;
use crate::resources::*;

use specs::world::*;
use specs::*;
use std::time::Instant;

pub struct ShootingSystem;

impl<'a> System<'a> for ShootingSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, LazyUpdate>,
    Read<'a, GameSettings>,
    Write<'a, PhysicsWorld>,
    ReadStorage<'a, PositionComponent>,
    WriteStorage<'a, ShootingComponent>,
  );

  fn run(
    &mut self,
    (entities, lazy, game_settings, mut physics, position, mut shooting): Self::SystemData,
  ) {
    for (position, shooting) in (&position, &mut shooting).join() {
      let now = Instant::now();
      let duration_since_last_shot = now.duration_since(shooting.last_shot_at);

      if duration_since_last_shot >= shooting.every {
        shooting.last_shot_at = now;

        let position_component = PositionComponent {
          x: position.x,
          y: position.y,
          angle: position.angle,
        };

        create_projectile(
          &entities,
          &lazy,
          &game_settings,
          &mut physics,
          position_component,
        );
      }
    }
  }
}
