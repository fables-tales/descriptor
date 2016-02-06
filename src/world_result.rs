use example_group::example_group_result::ExampleGroupResult;

pub struct WorldResult {
    results: Vec<ExampleGroupResult>,
}

impl WorldResult {
    pub fn new(results: Vec<ExampleGroupResult>) -> WorldResult {
        WorldResult {
            results: results,
        }
    }

    pub fn failed(&self) -> bool {
        self.results.iter().any(|r| r.failed())
    }

    pub fn results(&self) -> &[ExampleGroupResult] {
        &self.results
    }
}
