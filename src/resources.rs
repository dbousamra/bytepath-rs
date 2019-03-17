use nphysics2d::world::World;

#[derive(Debug, Default)]
pub struct UpdateTime(pub f32);

pub type PhysicsWorld = World<f32>;
