#![feature(pattern_parentheses)]

extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate nphysics_testbed2d;
extern crate specs;

use nphysics_testbed2d::Testbed;
use specs::prelude::*;

//struct PhysicsWorld(std::cell::RefCell<nphysics2d::world::World<f32>>);
struct PhysicsWorld(nphysics2d::world::World<f32>);

impl Default for PhysicsWorld {
    fn default() -> PhysicsWorld {
        //PhysicsWorld(std::cell::RefCell::new(nphysics2d::world::World::new()))
        PhysicsWorld(nphysics2d::world::World::new())
    }
}

struct DummySystem {
    counter: u32,
}

impl DummySystem {
    fn new() -> DummySystem {
        DummySystem { counter: 0 }
    }
}
impl<'a> System<'a> for DummySystem {
    type SystemData = (Write<'a, PhysicsWorld>,);

    fn run(&mut self, data: Self::SystemData) {
        if self.counter < 10 {
            let (mut physics_world) = data;

            let physics_world = &mut (physics_world.0).0;
            use na::{Isometry2, Vector2};
            use ncollide2d::shape::{Cuboid, ShapeHandle};
            use nphysics2d::volumetric::Volumetric;

            // Adapted from http://nphysics.org/rigid_body_simulations_with_contacts/ and some demo examples.
            // If something shorter exists, please share
            let cuboid = ShapeHandle::new(Cuboid::new(Vector2::new(1.0, 2.0)));
            let local_inertia = cuboid.inertia(1.0);
            let local_center_of_mass = cuboid.center_of_mass();
            physics_world.add_rigid_body(
                Isometry2::new(Vector2::x() * 2.0, na::zero()),
                local_inertia,
                local_center_of_mass,
            );
            self.counter += 1;
        }
        println!("dummy System");
    }
}

fn main() {
    // nphysics initialization
    let physics_world = nphysics2d::world::World::new();

    // Specs initialization
    let ecs_world = std::cell::RefCell::new(specs::World::new());
    ecs_world
        .borrow_mut()
        .add_resource(PhysicsWorld(physics_world));
    let dispatcher = DispatcherBuilder::new()
        .with(DummySystem::new(), "dummy_system", &[])
        .build();

    let dispatcher = std::cell::RefCell::new(dispatcher);

    // Testbed initialization
    let mut testbed = Testbed::new(
        // FIXME: This cannot compile because it's moved into PhysicsWorld resource.
        physics_world,
    );
    testbed.add_callback(move |_, _, _| {
        let mut ecs_world = ecs_world.borrow_mut();
        dispatcher.borrow_mut().dispatch(&ecs_world.res);
        ecs_world.maintain();
    });
    testbed.run();
}
