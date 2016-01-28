use std::boxed::FnBox;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, spawn};

use world_state;

pub struct ExampleResultInner {
    _description: String,
}

impl ExampleResultInner {
    pub fn new(description: String) -> ExampleResultInner {
        ExampleResultInner {
            _description: description,
        }
    }
}


pub type ExampleResult = Result<ExampleResultInner, ExampleResultInner>;
pub type ExampleRecoveryBlock = Box<FnBox() -> thread::Result<()> + Send + 'static>;

pub struct Example {
    description: String,
    block: ExampleRecoveryBlock,
}

impl Example {
    pub fn new(description: String, block: ExampleRecoveryBlock) -> Example {
        Example {
            description: description,
            block: block,
        }
    }

    pub fn spawn(self, state: Arc<Mutex<world_state::WorldState>>) -> JoinHandle<ExampleResult> {
        spawn(move || self.run(state))
    }

    fn run(self, state: Arc<Mutex<world_state::WorldState>>) -> ExampleResult {
        let block_result = (self.block)();

        let result = match block_result {
            Err(_) => Err(ExampleResultInner::new(self.description)),
            Ok(_) => Ok(ExampleResultInner::new(self.description)),
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
