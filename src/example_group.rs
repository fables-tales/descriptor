#![feature(catch_panic, fnbox)]
use std::boxed::FnBox;
use std::any::{Any};
use std::sync::{Arc, Mutex};
use std::fmt;
use std::thread::{JoinHandle, spawn, catch_panic};

use world_state;
use util::await_handles;
pub use reporter;

pub type ExampleResult = Result<(), Box<Any + Send>>;
pub type Examples = Vec<Box<FnBox(Arc<Mutex<world_state::WorldState>>) -> ExampleResult + Send + 'static>>;

pub struct ExampleGroup {
    pub description: String,
    pub examples: Examples,
}

impl fmt::Debug for ExampleGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group with description {}>", self.description)
    }
}

impl ExampleGroup {
    pub fn it<F>(&mut self, description: &str, example_definition_block: F) where F: Fn() + Send + 'static {
        self.examples.push(Box::new(move |state: Arc<Mutex<world_state::WorldState>>| {
            let result = catch_panic(example_definition_block);

            if result.is_err() {
                state.lock().unwrap().reporter.example_failed();
            } else {
                state.lock().unwrap().reporter.example_passed();
            }

            return result;
        }));
    }

    pub fn run(mut self, state: Arc<Mutex<world_state::WorldState>>, block: Box<Fn(&mut ExampleGroup) + Send + 'static>) -> bool {
        block(&mut self);
        let running_examples = Self::build_running_examples(state, self.examples);

        let mut failed = false;

        let results = await_handles(running_examples);

        for result in results.into_iter() {
            if result.is_err() {
                failed = true;
            }
        }

        return failed;
    }

    fn build_running_examples(state: Arc<Mutex<world_state::WorldState>>, examples: Examples) -> Vec<JoinHandle<ExampleResult>> {
        examples.into_iter().map(|example| {
            let state = state.clone();
            spawn(move || example(state))
        }).collect()
    }
}
