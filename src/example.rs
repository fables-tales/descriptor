use std::boxed::FnBox;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};
use std::any::{Any};

use world_state;

pub type ExampleResult = Result<(), Box<Any + Send + 'static>>;
pub type ExampleRecoveryBlock = Box<FnBox() -> Result<(),  Box<Any + Send + 'static>> + Send + 'static>;

pub struct Example {
    _description: String,
    block: ExampleRecoveryBlock,
}

impl Example {
    pub fn new(description: String, block: ExampleRecoveryBlock) -> Example {
        Example {
            _description: description,
            block: block,
        }
    }

    pub fn spawn(self, state: Arc<Mutex<world_state::WorldState>>) -> JoinHandle<ExampleResult> {
        spawn(move || self.run(state))
    }

    fn run(self, state: Arc<Mutex<world_state::WorldState>>) -> ExampleResult {
        let result = (self.block)();

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
