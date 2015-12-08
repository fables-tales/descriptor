#![feature(catch_panic)]
#[macro_use]
extern crate lazy_static;
use std::string::ToString;
use std::thread::{JoinHandle, spawn, catch_panic};
use std::sync::*;
use std::any::{Any};


pub struct ExampleGroup {
    description: String,
    running_examples: Vec<JoinHandle<Result<(), Box<Any + Send>>>>,
}

pub struct ExampleGroupAndBlock {
    group: ExampleGroup,
    block: Box<Fn(&mut ExampleGroup) + Sync + Send + 'static>,
}

pub struct Example {
    description: String,
}

impl ExampleGroup {
    pub fn it<F>(&mut self, description: &'static str, example_definition_block: F) where F: Fn() + Sync + Send + 'static {
        self.running_examples.push(spawn(move || {
            let result = catch_panic(move || {
                example_definition_block();
            });

            with_world(|world| {
                if result.is_err() {
                    world.reporter.example_failed();
                } else {
                    world.reporter.example_passed();
                }
            });

            return result;
        }));
    }

    fn block_on_all_examples(self) -> Result<(), ()> {
        let mut failed = false;
        for jh in self.running_examples.into_iter() {
            if jh.join().unwrap().is_err() {
                failed = true;
            }
        }

        if failed {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn run(mut self, block: Box<Fn(&mut ExampleGroup) + Send + Sync + 'static>) -> bool {
        block(&mut self);

        let result = self.block_on_all_examples();
        return result.is_ok();
    }
}

pub struct Reporter;

impl Reporter {
    pub fn example_failed(&self) {
        print!("F");
    }

    pub fn example_passed(&self) {
        print!(".");
    }
}

struct World {
    reporter: Reporter,
    failed: bool,
    example_groups: Vec<ExampleGroupAndBlock>,
}

fn with_world<F>(blk: F) where F: FnOnce(&mut World) -> () {
    let c = WORLD.clone();
    let mut guard = c.lock().unwrap();
    blk(&mut guard);
}

impl World {
}


lazy_static! {
    static ref WORLD: Arc<Mutex<World>> = Arc::new(Mutex::new(World { failed: false, reporter: Reporter, example_groups: Vec::new() }));
}

pub fn describe<F>(description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Sync + Send + 'static {
    with_world(|world| {
        world.example_groups.push(
            ExampleGroupAndBlock {
                group: ExampleGroup {
                    description: description.to_string(),
                    running_examples: Vec::new(),
                },
                block: Box::new(example_group_definition_block)
            }
        );
    });
}

fn get_examples_from_world() -> Vec<ExampleGroupAndBlock> {
    let mut result = Vec::new();

    with_world(|world| {
        while !world.example_groups.is_empty() {
            result.push(world.example_groups.remove(0));
        }
    });

    return result;

}

pub fn descriptor_main() {
    let mut example_groups = get_examples_from_world();
    let mut join_handles = Vec::new();

    while !example_groups.is_empty() {
        let example_group_and_block = example_groups.remove(0);
        let example_group = example_group_and_block.group;
        let block = example_group_and_block.block;
        join_handles.push(spawn(|| -> Result<(), ()> {
            if example_group.run(block) {
                return Ok(());
            } else {
                return Err(());
            }
        }));
    }

    for join_handle in join_handles.into_iter() {
        let result = join_handle.join().unwrap();
        if result.is_err() {
            with_world(|world| {
                world.failed = true;
            });
        }
    }
    with_world(|world| {
        println!("{}", world.failed);
    });
}
