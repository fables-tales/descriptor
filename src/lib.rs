#![feature(recover, std_panic, panic_handler, fnbox)]
#[macro_use]
extern crate lazy_static;

pub mod example_group;

mod macros;
mod util;
mod reporter;
mod world;
mod world_result;
mod world_state;
mod example;
mod example_group_and_block;

pub use util::SourceLocation;

use std::sync::{Arc, Mutex};
use world::World;
use example_group::example_group::ExampleGroup;


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

pub fn describe<F>(description: &str, source_location: SourceLocation, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Send + 'static {
    with_world(|world| {
        world.describe(description, source_location, example_group_definition_block);
    });
}

pub fn descriptor_main() {
    consuming_world(|world| world.run());
}
