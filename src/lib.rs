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

struct Example {
    description: String,
}

impl ExampleGroup {
    pub fn it<F>(&mut self, description: &'static str, example_definition_block: F) where F: Fn() + Sync + Send + 'static {
        self.running_examples.push(spawn(move || {
            let result = catch_panic(move || {
                example_definition_block();
            });

            let c = WORLD.clone();
            let world = c.lock().unwrap();
            if result.is_err() {
                world.reporter.example_failed();
            } else {
                world.reporter.example_passed();
            }

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
}


lazy_static! {
    static ref WORLD: Arc<Mutex<World>> = Arc::new(Mutex::new(World { failed: false, reporter: Reporter }));
}

pub fn describe<F>(description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Sync + Send + 'static {
    let mut eg = ExampleGroup { description: description.to_string(), running_examples: Vec::new() };
    example_group_definition_block(&mut eg);

    let status = eg.block_on_all_examples();

    if status.is_err() {
        let c = WORLD.clone();
        let mut world = c.lock().unwrap();
        world.failed = true;
    }
}

pub fn descriptor_main() {
    println!("{}", WORLD.lock().unwrap().failed);
}
