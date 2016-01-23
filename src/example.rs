use std::boxed::FnBox;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};
use std::any::{Any};
use std::panic::{recover, RecoverSafe};

use world_state;

pub type ExampleResult = Result<(), Box<Any + Send>>;
pub type ExampleBlock = Box<FnBox(Arc<Mutex<world_state::WorldState>>) -> ExampleResult + Send + 'static>;

pub struct Example<T: Fn() + Send + RecoverSafe + 'static> {
    _description: String,
    block: ExampleBlock,
}

impl <T> Example<T> {
    pub fn new<T>(description: String, block: ExampleBlock, definition_block: T) -> Example<T> where T: Fn() + Send + RecoverSafe + 'static {
        Example {
            _description: description,
            block: block,
            definition_block: definition_block,
        }
    }

    pub fn spawn(self, state: Arc<Mutex<world_state::WorldState>>) -> JoinHandle<ExampleResult> {
        spawn(move || self.run(state))
    }

    fn run(self, state: Arc<Mutex<world_state::WorldState>>) -> ExampleResult {
        let block = self.block;
        block(state)
    }
}
