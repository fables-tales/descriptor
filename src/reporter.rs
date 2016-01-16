use std::io::{self,Write};
#[derive(Debug)]
pub struct Reporter;

impl Reporter {
    pub fn example_failed(&self) {
        print!("F");
        io::stdout().flush();
    }

    pub fn example_passed(&self) {
        print!(".");
        io::stdout().flush();
    }
}

