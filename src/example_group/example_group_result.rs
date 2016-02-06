use example::{ExampleResult, ExampleResultInner};
use example_group::example_group::ExampleGroupMetadata;
use util::borrow_any_is_err;

pub struct ExampleGroupResult {
    results: Vec<ExampleResult>,
    pub metadata: ExampleGroupMetadata,
}

impl ExampleGroupResult {
    pub fn new(metadata: ExampleGroupMetadata, results: Vec<ExampleResult>) -> ExampleGroupResult {
        ExampleGroupResult {
            metadata: metadata,
            results: results,
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
