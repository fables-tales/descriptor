#![feature(catch_panic, fnbox)]
#[macro_use]
extern crate lazy_static;
mod util;
pub mod reporter;
mod world_state;
pub mod example_group;
mod example_group_and_block;

use std::string::ToString;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};

use util::{await_handles, any_is_err};
use example_group_and_block::ExampleGroupAndBlock;

#[derive(Debug)]
struct World {
    state: Arc<Mutex<world_state::WorldState>>,
    example_groups: Vec<ExampleGroupAndBlock>,
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

impl World {
    fn new() -> World {
        World {
            state: Arc::new(Mutex::new(world_state::WorldState {
                failed: false,
                reporter: reporter::Reporter,
            })),
            example_groups: Vec::new(),
        }
    }

    fn describe<F>(&mut self, description: &str, example_group_definition_block: F) where F: Fn(&mut example_group::ExampleGroup) + Send + 'static {
        self.example_groups.push(
            ExampleGroupAndBlock {
                group: example_group::ExampleGroup {
                    description: description.to_string(),
                    examples: Vec::new(),
                },
                block: Box::new(example_group_definition_block)
            }
        );
    }

    fn run(self) -> world_state::WorldState {
        let join_handles: Vec<_> = World::create_example_group_join_handles(self.state.clone(), self.example_groups);
        let results = await_handles(join_handles);
        let failed = any_is_err(results);

        let state_guard = self.state.clone();
        let mut state = state_guard.lock().unwrap();
        state.failed = failed;


        world_state::WorldState {
            failed: state.failed,
            reporter: reporter::Reporter

        }
    }

    fn create_example_group_join_handles(state: Arc<Mutex<world_state::WorldState>>, example_groups: Vec<ExampleGroupAndBlock>) -> Vec<JoinHandle<Result<(), ()>>> {
        example_groups.into_iter().map(|egab| {
            egab.spawn(&state)
        }).collect()
    }

}

lazy_static! {
    static ref WORLD: Arc<Mutex<World>> = Arc::new(Mutex::new(World::new()));
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
