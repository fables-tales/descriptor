use std::thread::{JoinHandle};
pub fn await_handles<T>(join_handles: Vec<JoinHandle<T>>) -> Vec<T> {
    join_handles.into_iter().map(|jh| {
        jh.join().unwrap()
    }).collect()
}
