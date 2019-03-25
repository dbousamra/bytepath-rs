use nphysics2d::object::{BodyHandle, RigidBody};
use specs::world::*;
use specs::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;

pub struct BoundsSystem;

impl<'a> System<'a> for BoundsSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, LazyUpdate>,
    ReadStorage<'a, BoundsComponent>,
    ReadStorage<'a, ExplodeBoundsComponent>,
    ReadStorage<'a, PositionComponent>,
    WriteStorage<'a, GarbageComponent>,
  );

  fn run(
    &mut self,
    (entities, lazy, bounds, explode_bounds, position, mut garbage): Self::SystemData,
  ) {
    for (entity, bounds, position, garbage) in (&entities, &bounds, &position, &mut garbage).join()
    {
      let out_of_bounds_x = position.x < bounds.x_min as f32 || position.x > bounds.x_max as f32;
      let out_of_bounds_y = position.y < bounds.y_min as f32 || position.y > bounds.y_max as f32;

      if out_of_bounds_x || out_of_bounds_y {
        garbage.is_alive = false;

        let explode_bounds_component: Option<&ExplodeBoundsComponent> = explode_bounds.get(entity);
        if let Some(_) = explode_bounds_component {
          let position_component = PositionComponent {
            x: position.x,
            y: position.y,
            angle: 0.0,
          };
          create_out_of_bounds_explosion(&entities, &lazy, position_component);
        }
      }
    }
  }
}
