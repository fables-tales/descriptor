use example::ExampleResult;
use util::borrow_any_is_err;

pub struct ExampleGroupResult {
    results: Vec<ExampleResult>,
    pub description: String,
}

impl ExampleGroupResult {
    pub fn new(description: String, results: Vec<ExampleResult>) -> ExampleGroupResult {
        ExampleGroupResult {
            results: results,
            description: description,
        }
    }

    pub fn failed(&self) -> bool {
        borrow_any_is_err(&self.results)
    }
}
