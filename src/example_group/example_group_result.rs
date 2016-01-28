use example::ExampleResult;
use util::any_is_err;

pub struct ExampleGroupResult {
    pub failed: bool,
}

impl ExampleGroupResult {
    pub fn new(results: Vec<ExampleResult>) -> ExampleGroupResult {
        let failed = any_is_err(results);
        ExampleGroupResult {
            failed: failed
        }
    }

    pub fn failed(&self) -> bool {
        self.failed
    }
}
