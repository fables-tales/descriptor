pub struct ExampleGroupResult {
    pub failed: bool,
}

impl ExampleGroupResult {
    pub fn new(failed: bool) -> ExampleGroupResult {
        ExampleGroupResult {
            failed: failed
        }
    }
}
