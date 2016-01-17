use reporter;
use std::sync::Mutex;

#[derive(Debug)]
pub struct WorldState {
    pub reporter: Box<reporter::Reporter + Send>,
    pub failed: bool,
}

impl WorldState {
    pub fn new(reporter: Box<reporter::Reporter + Send>) -> WorldState {
        WorldState {
            reporter: reporter,
            failed: false,
        }
    }
}
