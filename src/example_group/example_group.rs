use std::sync::{Arc, Mutex};
use std::fmt;
use std::thread::JoinHandle;
use std::panic::{recover, RecoverSafe};

use world_state;
use util::await_handles;
use example::{Example, ExampleResult};
use example_group::example_group_result::{ExampleGroupResult};

pub struct ExampleGroup {
    description: String,
    examples: Vec<Example>,
}

impl fmt::Debug for ExampleGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group with description {}>", self.description)
    }
}

impl ExampleGroup {
    pub fn new(description: &str) -> ExampleGroup {
        ExampleGroup {
            description: description.to_string(),
            examples: Vec::new(),
        }
    }

    pub fn it<F>(&mut self, description: &str, example_definition_block: F) where F: Fn() + Send + RecoverSafe + 'static {
        let recovery_proc = Box::new(|| recover(example_definition_block));
        let example = Example::new(description.into(), recovery_proc);

        self.examples.push(example);
    }

    pub fn run(mut self, state: Arc<Mutex<world_state::WorldState>>, block: Box<Fn(&mut ExampleGroup) + Send + 'static>) -> ExampleGroupResult {
        block(&mut self);

        let running_examples = Self::build_running_examples(state, self.examples);
        let results = await_handles(running_examples);

        return ExampleGroupResult::new(results);
    }

    fn build_running_examples(state: Arc<Mutex<world_state::WorldState>>, examples: Vec<Example>) -> Vec<JoinHandle<ExampleResult>> {
        examples.into_iter().map(|example| {
            let state = state.clone();

            example.spawn(state)
        }).collect()
    }
}
