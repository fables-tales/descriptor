use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};

use example_group;
use world_state;

pub struct ExampleGroupAndBlock {
    pub group: example_group::ExampleGroup,
    pub block: Box<Fn(&mut example_group::ExampleGroup) + Send + 'static>,
}

impl ExampleGroupAndBlock {
    pub fn spawn(self, state: &Arc<Mutex<world_state::WorldState>>) -> JoinHandle<Result<(), ()>> {
        let state = state.clone();
        let group = self.group;
        let block = self.block;

        spawn(|| -> Result<(), ()> {
            if group.run(state, block) {
                Ok(())
            } else {
                Err(())
            }
        })
    }
}

impl fmt::Debug for ExampleGroupAndBlock {
    fn fmt(&self, formatter: &mut fmt::Formatter) ->  fmt::Result {
        write!(formatter, "<Example group and block with example_group {:#?}>", self.group)
    }
}
