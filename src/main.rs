extern crate nphysics_testbed2d;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;

use nphysics2d::world::World;
use nphysics_testbed2d::Testbed;

pub struct Data {
  pub data: String,
}

fn main() {
  let data = Data { data: "test" };
  let testbed = Testbed::new(World::new_empty());
  testbed.add_callback( move |_,_,_| {
    println!("non static data from outside the callback block: {}", data.data);
  }
  testbed.run();
}
