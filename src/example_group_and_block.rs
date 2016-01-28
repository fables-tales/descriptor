use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};

use example_group::example_group::ExampleGroup;
use world_state::WorldState;
use example_group::example_group_result::ExampleGroupResult;

pub struct ExampleGroupAndBlock {
    pub group: ExampleGroup,
    pub block: Box<Fn(&mut ExampleGroup) + Send + 'static>,
}

impl ExampleGroupAndBlock {
    pub fn new(group: ExampleGroup, block: Box<Fn(&mut ExampleGroup) + Send + 'static>) -> ExampleGroupAndBlock {
        ExampleGroupAndBlock {
            group: group,
            block: block,
        }
    }

    pub fn spawn(self, state: &Arc<Mutex<WorldState>>) -> JoinHandle<ExampleGroupResult> {
        let state = state.clone();
        let group = self.group;
        let block = self.block;

        spawn(|| group.run(state, block))
    }
}

impl fmt::Debug for ExampleGroupAndBlock {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group and block with example_group {:#?}>", self.group)
    }
}
