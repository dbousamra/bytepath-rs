use crate::components::*;
use crate::entities::*;
use crate::resources::*;

use easer::functions::*;
use specs::world::*;
use specs::*;
use std::ops::Add;
use std::time::Duration;

pub struct TweenSystem;

impl<'a> System<'a> for TweenSystem {
  type SystemData = (
    Read<'a, EntitiesRes>,
    Read<'a, UpdateTime>,
    WriteStorage<'a, TweenComponent>,
    WriteStorage<'a, MeshComponent>,
  );

  fn run(&mut self, (entities, update_time, mut tweens, mut mesh): Self::SystemData) {
    for (entity, tween) in (&entities, &mut tweens).join() {
      if tween.elapsed.as_millis() <= tween.duration.as_millis() {
        let dt = update_time.0;
        tween.elapsed = tween.elapsed.add(dt);
        match &tween.tween_type {
          Tween::SizeTween {
            ease,
            starting,
            ending,
          } => {
            let tweened_value = if starting > ending {
              let tweened_value = match ease {
                Ease::Cubic => Cubic::ease_in(
                  tween.elapsed.as_millis() as f32,
                  *ending,
                  *starting,
                  tween.duration.as_millis() as f32,
                ),
              };
              *starting - tweened_value
            } else {
              match ease {
                Ease::Cubic => Cubic::ease_in(
                  tween.elapsed.as_millis() as f32,
                  *ending,
                  *starting,
                  tween.duration.as_millis() as f32,
                ),
              }
            };

            let mesh: Option<&mut MeshComponent> = mesh.get_mut(entity);

            if let Some(mesh) = mesh {
              (mesh).draw_param = ggez::graphics::DrawParam {
                scale: ggez::graphics::Point2::new(tweened_value, tweened_value),
                ..mesh.draw_param
              };
            }
          }
        };
      }
    }
  }
}
