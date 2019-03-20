use specs::{ReadStorage, System};

use crate::components::*;

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
  type SystemData = ReadStorage<'a, Mesh>;

  fn run(&mut self, mesh: Self::SystemData) {
    use specs::Join;

    for mesh in mesh.join() {
      println!("Hello, {:?}", &mesh);
    }
  }
}
