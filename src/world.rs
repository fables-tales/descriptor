use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::panic;

use util::{await_handles, any_is_err};
use example_group_and_block::ExampleGroupAndBlock;
use reporter::{ProgressReporter, Reporter};
use example_group::example_group::ExampleGroup;
use world_state::WorldState;
use world_result::WorldResult;

#[derive(Debug)]
pub struct World {
    state: Arc<Mutex<WorldState>>,
    example_groups: Vec<ExampleGroupAndBlock>,
}

fn silencing_panics<A, T>(block: A) -> T where A: FnOnce() -> T {
    let orig_panic_handler = panic::take_handler();
    panic::set_handler(|_| ());

    let result = block();

    panic::set_handler(move |arg| (*orig_panic_handler)(arg));

    result
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

    pub fn run(self) -> WorldResult {
        let state = self.state.clone();

        let failed = self.example_run_result();
        let result = WorldResult::new(failed);

        World::report_result(&result, &state.lock().unwrap().reporter);

        result
    }

    fn report_result(result: &WorldResult, reporter: &Box<Reporter + Send + 'static>) {
        let report_result = reporter.report_result(&result);
        if report_result.is_err() {
            panic!("work out what to do here");
        }
    }

    fn example_run_result(self) -> bool {
        let results = silencing_panics(move || {
            let join_handles: Vec<_> = World::create_example_group_join_handles(self.state.clone(), self.example_groups);
            await_handles(join_handles)
        });

        let failed = any_is_err(results);

        failed
    }


    fn create_example_group_join_handles(state: Arc<Mutex<WorldState>>, example_groups: Vec<ExampleGroupAndBlock>) -> Vec<JoinHandle<Result<(), ()>>> {
        example_groups.into_iter().map(|egab| egab.spawn(&state)).collect()
    }
}
