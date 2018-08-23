extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate nphysics_testbed2d;
extern crate specs;

use nphysics_testbed2d::Testbed;
use nphysics_testbed2d::WorldOwnerShared;
use specs::prelude::*;
use na::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::volumetric::Volumetric;
use nphysics2d::object::{Material};
use nphysics_testbed2d::WorldOwner;

//struct PhysicsWorld(std::cell::RefCell<nphysics2d::world::World<f32>>);
#[derive(Clone)]
struct PhysicsWorld(WorldOwnerShared);

impl Default for PhysicsWorld {
    fn default() -> PhysicsWorld {
        //PhysicsWorld(std::cell::RefCell::new(nphysics2d::world::World::new()))
        PhysicsWorld(WorldOwnerShared::new(nphysics2d::world::World::new()))
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

const COLLIDER_MARGIN: f32 = 0.01;

impl<'a> System<'a> for DummySystem {
    type SystemData = (Write<'a, PhysicsWorld>,);

    fn run(&mut self, data: Self::SystemData) {
        if self.counter < 10 {

            let material = Material::default();
                        let geom = ShapeHandle::new(Ball::new(0.09));
            let inertia = geom.inertia(1.0);
            let center_of_mass = geom.center_of_mass();

            let mut physics_world = data.0;

            let physics_world = &mut (physics_world.0).get_mut();
            /*
             * Create the rigid body.
             */
            let pos = Isometry2::new(Vector2::new(0.0, 0.0), 0.0);
            let handle = physics_world.add_rigid_body(pos, inertia, center_of_mass);

            /*
             * Create the collider.
             */
            physics_world.add_collider(
                COLLIDER_MARGIN,
                geom.clone(),
                handle,
                Isometry2::identity(),
                material.clone(),
            );
            self.counter += 1;
        }
        println!("dummy System");
    }
}


fn main() {
    // nphysics initialization
    let mut physics_world = PhysicsWorld(WorldOwnerShared::new(nphysics2d::world::World::new()));


    // Materials.
    let material = Material::default();
    
/*
     * Create the boxes
     */
    let num = 25;
    let rad = 0.1;
    let shift = rad * 2.0 + 0.002;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;

    let geom = ShapeHandle::new(Ball::new(rad - COLLIDER_MARGIN));
    let inertia = geom.inertia(1.0);
    let center_of_mass = geom.center_of_mass();

    for i in 0usize..num {
        for j in 0..num {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery;

            /*
             * Create the rigid body.
             */
            let pos = Isometry2::new(Vector2::new(x, y), 0.0);
            let handle = physics_world.0.get_mut().add_rigid_body(pos, inertia, center_of_mass);

            /*
             * Create the collider.
             */
            physics_world.0.get_mut().add_collider(
                COLLIDER_MARGIN,
                geom.clone(),
                handle,
                Isometry2::identity(),
                material.clone(),
            );
        }
    }

    // Specs initialization
    let ecs_world = std::cell::RefCell::new(specs::World::new());
    ecs_world
        .borrow_mut()
        .add_resource(physics_world.clone());
    let dispatcher = DispatcherBuilder::new()
        .with(DummySystem::new(), "dummy_system", &[])
        .build();

    let dispatcher = std::cell::RefCell::new(dispatcher);

    // Testbed initialization
    let mut testbed = Testbed::new_with_world_owner(
        // FIXME: This cannot compile because it's moved into PhysicsWorld resource.
        Box::new(physics_world.clone().0),
    );
    testbed.add_callback(move |_, _, _| {
        let mut ecs_world = ecs_world.borrow_mut();
        dispatcher.borrow_mut().dispatch(&ecs_world.res);
        ecs_world.maintain();
    });
    testbed.run();
}
