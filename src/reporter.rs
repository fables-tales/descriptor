use std::io::{self,Write};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ProgressReporter;

pub trait Reporter: Debug {
    fn example_failed(&self) -> Result<(), Box<::std::error::Error>>;
    fn example_passed(&self) -> Result<(), Box<::std::error::Error>>;
}

enum Colors {
    Green,
    Red,
}

fn colorize(string: &str, color: Colors) -> String {
    let code = pick_code(color);
    format!("\x1B[{}m{}\x1b[0m", code, string)
}

fn pick_code(color: Colors) -> &'static str {
    match color {
        Colors::Green => "32",
        Colors::Red => "31",
    }
}

impl Reporter for ProgressReporter {
    fn example_failed(&self) -> Result<(), Box<::std::error::Error>> {
        print!("{}", colorize("F", Colors::Red));
        io::stdout().flush().map_err(|e| e.into())
    }

    fn example_passed(&self) -> Result<(), Box<::std::error::Error>> {
        print!("{}", colorize(".", Colors::Green));
        io::stdout().flush().map_err(|e| e.into())
    }


}


#[derive(Debug)]
pub struct SuiteCompleteReporter;

impl Reporter for SuiteCompleteReporter{
    fn example_failed(&self) -> Result<(), Box<::std::error::Error>> {
        panic!("The suite is complete, you cannot call further methods on the reporter");
    }

    fn example_passed(&self) -> Result<(), Box<::std::error::Error>> {
        panic!("The suite is complete, you cannot call further methods on the reporter");
    }
}
