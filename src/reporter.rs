use std::io::{self,Write};
use std::fmt::Debug;

use world_result::WorldResult;

#[derive(Debug)]
pub struct ProgressReporter;

pub trait Reporter: Debug {
    fn example_failed(&self) -> Result<(), Box<::std::error::Error>>;
    fn example_passed(&self) -> Result<(), Box<::std::error::Error>>;
    fn report_result(&self, &WorldResult) -> Result<(), Box<::std::error::Error>>;
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

    fn report_result(&self, result: &WorldResult) -> Result<(), Box<::std::error::Error>> {
        println!("");
        println!("Suite failed? {}", result.failed());
        println!("Example group failures:");

        for r in result.results().iter().filter(|r| r.failed()) {
            println!("{}", r.description);
        };

        io::stdout().flush().map_err(|e| e.into())
    }
}
