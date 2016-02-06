use example::{ExampleResult, ExampleResultInner};
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

    pub fn failed_examples(&self) -> Vec<&ExampleResultInner> {
        self.results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .collect()
    }
}
