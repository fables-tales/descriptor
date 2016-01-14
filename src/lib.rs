#![feature(catch_panic)]
#[macro_use]
extern crate lazy_static;
use std::string::ToString;
use std::thread::{JoinHandle, spawn, catch_panic};
use std::sync::*;
use std::any::{Any};
use std::io::{self, Write};



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
    pub fn it<F>(&mut self, description: &str, example_definition_block: F) where F: Fn() + Sync + Send + 'static {
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
        io::stdout().flush();
    }

    pub fn example_passed(&self) {
        print!(".");
        io::stdout().flush();
    }
}

struct World {
    reporter: Reporter,
    failed: bool,
    example_groups: Vec<ExampleGroupAndBlock>,
}

fn with_world<F, T>(blk: F) -> T where F: FnOnce(&mut World) -> T {
    let c = WORLD.clone();
    let mut guard = c.lock().unwrap();
    blk(&mut guard)
}

impl World {
    fn describe<F>(&mut self, description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Sync + Send + 'static {
        self.example_groups.push(
            ExampleGroupAndBlock {
                group: ExampleGroup {
                    description: description.to_string(),
                    running_examples: Vec::new(),
                },
                block: Box::new(example_group_definition_block)
            }
        );
    }
}


lazy_static! {
    static ref WORLD: Arc<Mutex<World>> = Arc::new(Mutex::new(World { failed: false, reporter: Reporter, example_groups: Vec::new() }));
}

pub fn describe<F>(description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Sync + Send + 'static {
    with_world(|world| {
        world.describe(description, example_group_definition_block);
    });
}

fn get_examples_from_world() -> Vec<ExampleGroupAndBlock> {
    let mut result = Vec::new();

    with_world(|world| {
        ::std::mem::replace(&mut world.example_groups, result)
    })
}

pub fn descriptor_main() {
    let mut example_groups = get_examples_from_world();
    let join_handles: Vec<JoinHandle<Result<(), ()>>> = example_groups.into_iter().map({ |ExampleGroupAndBlock { group: example_group, block: block }|
        spawn(|| -> Result<(), ()> {
            if example_group.run(block) {
                Ok(())
            } else {
                Err(())
            }
        })
    }).collect();

    let results = join_handles.into_iter().map(|jh| { jh.join().unwrap() });
    let failed = results.into_iter().any(|r| { r.is_err() });

    with_world(|world| {
        world.failed = failed;
        println!("{}", world.failed);
    });
}
