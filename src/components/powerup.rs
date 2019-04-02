use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub enum PowerUp {
  Ammo,
  Boost,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct PowerUpComponent {
  pub variant: PowerUp,
}
