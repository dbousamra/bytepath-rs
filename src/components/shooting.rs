use specs::{Component, VecStorage};
use std::time::{Duration, Instant};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ShootingComponent {
  pub every: Duration,
  pub last_shot_at: Instant,
}
