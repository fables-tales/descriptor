use std::sync::{Arc, Mutex};
use std::fmt;
use std::thread::JoinHandle;
use std::panic::{recover, RecoverSafe, UnwindSafe};

use world_state;
use util::{await_handles, SourceLocation};
use example::{Example, ExampleResult};
use example_group::example_group_result::{ExampleGroupResult};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct ExampleGroupMetadata {
    pub description: String,
    pub source_location: SourceLocation,
}

impl Display for ExampleGroupMetadata {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "`{}` at {}", self.description, self.source_location)
    }
}

pub struct ExampleGroup {
    examples: Vec<Example>,
    pub metadata: ExampleGroupMetadata
}

impl fmt::Debug for ExampleGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group with metadata {:?}>", self.metadata)
    }
}

impl ExampleGroup {
    pub fn new(description: &str, source_location: SourceLocation) -> ExampleGroup {
        ExampleGroup {
            examples: Vec::new(),
            metadata: ExampleGroupMetadata {
                description: description.to_string(),
                source_location: source_location,
            }
        }
    }

    pub fn it<F>(&mut self, description: &str, source_location: SourceLocation, example_definition_block: F) where F: Fn() + Send + RecoverSafe + UnwindSafe + 'static {
        let recovery_proc = Box::new(|| recover(example_definition_block));
        let example = Example::new(description.into(), source_location, recovery_proc);

        self.examples.push(example);
    }

    pub fn run(mut self, state: Arc<Mutex<world_state::WorldState>>, block: Box<Fn(&mut ExampleGroup) + Send + 'static>) -> ExampleGroupResult {
        block(&mut self);

        let running_examples = Self::build_running_examples(state, self.examples);
        let results = await_handles(running_examples);

        return ExampleGroupResult::new(self.metadata, results);
    }

    fn build_running_examples(state: Arc<Mutex<world_state::WorldState>>, examples: Vec<Example>) -> Vec<JoinHandle<ExampleResult>> {
        examples.into_iter().map(|example| {
            let state = state.clone();

            example.spawn(state)
        }).collect()
    }
}
