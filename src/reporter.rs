use std::io::{self,Write};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ProgressReporter;

pub trait Reporter: Debug {
    fn example_failed(&self);
    fn example_passed(&self);
}

impl Reporter for ProgressReporter {
    fn example_failed(&self) {
        print!("F");
        io::stdout().flush();
    }

    fn example_passed(&self) {
        print!(".");
        io::stdout().flush();
    }
}


#[derive(Debug)]
pub struct SuiteCompleteReporter;

impl Reporter for SuiteCompleteReporter{
    fn example_failed(&self) {
        panic!("The suite is complete, you cannot call further methods on the reporter");
    }

    fn example_passed(&self) {
        panic!("The suite is complete, you cannot call further methods on the reporter");
    }
}
