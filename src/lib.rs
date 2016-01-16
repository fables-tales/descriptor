#![feature(catch_panic, fnbox)]
#[macro_use]
extern crate lazy_static;
use std::boxed::FnBox;
use std::string::ToString;
use std::thread::{JoinHandle, spawn, catch_panic};
use std::sync::*;
use std::any::{Any};
use std::fmt;
use std::io::Write;

mod reporter;

pub struct ExampleGroup {
    description: String,
    examples: Vec<Box<FnBox(Arc<Mutex<WorldState>>) -> Result<(), Box<Any + Send>> + Send + 'static>>,
}

impl fmt::Debug for ExampleGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group with description {}>", self.description)
    }
}

pub struct ExampleGroupAndBlock {
    group: ExampleGroup,
    block: Box<Fn(&mut ExampleGroup) + Send + 'static>,
}

impl fmt::Debug for ExampleGroupAndBlock {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group and block with example_group {:#?}>", self.group)
    }
}

#[derive(Debug)]
pub struct Example {
    description: String,
}

#[derive(Debug)]
struct World {
    state: Arc<Mutex<WorldState>>,
    example_groups: Vec<ExampleGroupAndBlock>,
}

#[derive(Debug)]
struct WorldState {
    reporter: reporter::Reporter,
    failed: bool,
}

impl ExampleGroup {
    pub fn it<F>(&mut self, description: &str, example_definition_block: F) where F: Fn() + Send + 'static {
        self.examples.push(Box::new(move |state: Arc<Mutex<WorldState>>| {
            let result = catch_panic(example_definition_block);

            if result.is_err() {
                state.lock().unwrap().reporter.example_failed();
            } else {
                state.lock().unwrap().reporter.example_passed();
            }

            return result;
        }));
    }

    fn run(mut self, state: Arc<Mutex<WorldState>>, block: Box<Fn(&mut ExampleGroup) + Send + 'static>) -> bool {
        block(&mut self);
        let running_examples: Vec<_> = self.examples.into_iter().map(|example| {
            let state = state.clone();
            spawn(move || example(state))
        }).collect();

        let mut failed = false;

        let results: Vec<_> = running_examples.into_iter().map(|jh| {
            jh.join()
        }).collect();

        for jh in results.into_iter() {
            if jh.unwrap().is_err() {
                failed = true;
            }
        }

        return failed;
    }
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
    fn describe<F>(&mut self, description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Send + 'static {
        self.example_groups.push(
            ExampleGroupAndBlock {
                group: ExampleGroup {
                    description: description.to_string(),
                    examples: Vec::new(),
                },
                block: Box::new(example_group_definition_block)
            }
        );
    }

    fn run(self) -> WorldState {
        let join_handles: Vec<_> = World::create_example_group_join_handles(self.state.clone(), self.example_groups);
        let results = join_handles.into_iter().map(|jh| jh.join().unwrap());
        let failed = results.into_iter().any(|r| { r.is_err() });

        let state_guard = self.state.clone();
        let mut state = state_guard.lock().unwrap();
        state.failed = failed;


        WorldState {
            failed: state.failed,
            reporter: reporter::Reporter

        }
    }

    fn create_example_group_join_handles(state: Arc<Mutex<WorldState>>, example_groups: Vec<ExampleGroupAndBlock>) -> Vec<JoinHandle<Result<(), ()>>> {
        example_groups.into_iter().map(|ExampleGroupAndBlock { group: example_group, block}| {
            let state = state.clone();
            spawn(|| -> Result<(), ()> {
                if example_group.run(state, block) {
                    Ok(())
                } else {
                    Err(())
                }
            })
        }).collect()
    }

    fn new() -> World {
        World {
            state: Arc::new(Mutex::new(WorldState {
                failed: false,
                reporter: reporter::Reporter,
            })),
            example_groups: Vec::new(),
        }
    }
}


lazy_static! {
    static ref WORLD: Arc<Mutex<World>> = Arc::new(Mutex::new(World::new()));
}

pub fn describe<F>(description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Send + 'static {
    with_world(|world| {
        world.describe(description, example_group_definition_block);
    });
}

pub fn descriptor_main() {
    let state = consuming_world(|world| world.run());
    println!("{}", state.failed);
}
