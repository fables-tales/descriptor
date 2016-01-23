use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::panic;

use util::{await_handles, any_is_err};
use example_group_and_block::ExampleGroupAndBlock;
use reporter::{ProgressReporter, SuiteCompleteReporter};
use example_group::ExampleGroup;
use world_state::WorldState;

#[derive(Debug)]
pub struct World {
    state: Arc<Mutex<WorldState>>,
    example_groups: Vec<ExampleGroupAndBlock>,
}

impl World {
    pub fn new() -> World {
        World {
            state: Arc::new(Mutex::new(WorldState::new(Box::new(ProgressReporter)))),
            example_groups: Vec::new(),
        }
    }

    pub fn describe<F>(&mut self, description: &str, example_group_definition_block: F) where F: Fn(&mut ExampleGroup) + Send + 'static {
        self.example_groups.push(
            ExampleGroupAndBlock::new(
                ExampleGroup::new(description),
                Box::new(example_group_definition_block),
            )
        );
    }

    pub fn run(self) -> WorldState {
        let orig_panic_handler = panic::take_handler();
        panic::set_handler(|_| ());

        let failed = self.example_run_result();

        panic::set_handler(move |arg| (*orig_panic_handler)(arg));

        World::build_result(failed)
    }

    fn build_result(failed: bool) -> WorldState {
        let mut state = WorldState::new(Box::new(SuiteCompleteReporter));
        state.failed = failed;
        state
    }

    fn example_run_result(self) -> bool {
        let join_handles: Vec<_> = World::create_example_group_join_handles(self.state.clone(), self.example_groups);
        let results = await_handles(join_handles);
        let failed = any_is_err(results);

        failed
    }

    fn create_example_group_join_handles(state: Arc<Mutex<WorldState>>, example_groups: Vec<ExampleGroupAndBlock>) -> Vec<JoinHandle<Result<(), ()>>> {
        example_groups.into_iter().map(|egab| egab.spawn(&state)).collect()
    }
}
