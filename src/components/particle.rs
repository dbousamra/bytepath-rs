use specs::{Component, VecStorage};

use crate::components::*;
use std::time::Duration;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ParticleEmitterComponent {
  pub frequency: Duration,
}
