use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ControllableComponent;
