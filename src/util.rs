use std::fmt::{Display, Formatter, Error};
use std::thread::{JoinHandle};

pub fn await_handles<T>(join_handles: Vec<JoinHandle<T>>) -> Vec<T> {
    join_handles.into_iter().map(|jh| {
        jh.join().unwrap()
    }).collect()
}

pub fn borrow_any_is_err<A, B>(results: &[Result<A, B>]) -> bool {
    results.iter().any(|r| r.is_err())
}

#[derive(Debug)]
pub struct SourceLocation {
    pub file: &'static str,
    pub line: u32,
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[{}:{}]", self.file, self.line)
    }
}

#[macro_export]
macro_rules! source_location {
    () => {{
        let (file, line) = (file!(), line!());
        $crate::SourceLocation {
            file: file,
            line: line,
        }
    }}
}
