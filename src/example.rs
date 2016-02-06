use std::boxed::FnBox;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, spawn};
use util::SourceLocation;
use std::fmt::{Display, Formatter, Error};

use world_state;

pub type ExampleResult = Result<ExampleMetadata, ExampleMetadata>;
pub type ExampleRecoveryBlock = Box<FnBox() -> thread::Result<()> + Send + 'static>;

pub struct ExampleMetadata {
    pub description: String,
    pub source_location: SourceLocation,
}

impl Display for ExampleMetadata {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "`{}` at {}", self.description, self.source_location)
    }
}

pub struct Example {
    block: ExampleRecoveryBlock,
    metadata: ExampleMetadata,
}

impl Example {
    pub fn new(description: String, source_location: SourceLocation, block: ExampleRecoveryBlock) -> Example {
        Example {
            block: block,
            metadata: ExampleMetadata {
                description: description,
                source_location: source_location,
            }
        }
    }

    pub fn spawn(self, state: Arc<Mutex<world_state::WorldState>>) -> JoinHandle<ExampleResult> {
        spawn(move || self.run(state))
    }

    fn run(self, state: Arc<Mutex<world_state::WorldState>>) -> ExampleResult {
        let block_result = (self.block)();

        let result = match block_result {
            Err(_) => Err(self.metadata),
            Ok(_) => Ok(self.metadata),
        };

        Example::report_result(&result, state);

        result
    }

    fn report_result(result: &ExampleResult, state: Arc<Mutex<world_state::WorldState>>) {
        let ref reporter = state.lock().unwrap().reporter;

        let reporting_result = if result.is_err() {
            reporter.example_failed()
        } else {
            reporter.example_passed()
        };

        if reporting_result.is_err() {
            panic!("work out what to do here");
        }
    }
}
