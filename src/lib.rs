#![feature(catch_panic, fnbox)]
#[macro_use]
extern crate lazy_static;
mod util;
mod reporter;
mod world_state;
pub mod example_group;
mod example_group_and_block;
mod world;

use std::sync::{Arc, Mutex};
use world::World;

lazy_static! {
    static ref WORLD: Arc<Mutex<World>> = Arc::new(Mutex::new(World::new()));
}

fn with_world<F, T>(blk: F) -> T where F: FnOnce(&mut World) -> T {
    let c = WORLD.clone();
    let mut guard = c.lock().unwrap();

    blk(&mut guard)
}

fn consuming_world<F, T>(blk: F) -> T where F: FnOnce(World) -> T {
    let guard = WORLD.clone();
    let mut world_current = guard.lock().unwrap();

    let mut world = World::new();

    std::mem::swap(&mut world, &mut world_current);

    blk(world)
}

pub fn describe<F>(description: &str, example_group_definition_block: F) where F: Fn(&mut example_group::ExampleGroup) + Send + 'static {
    with_world(|world| {
        world.describe(description, example_group_definition_block);
    });
}

pub fn descriptor_main() {
    let state = consuming_world(|world| world.run());
    println!("{}", state.failed);
}
