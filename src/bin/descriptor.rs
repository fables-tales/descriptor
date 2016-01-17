extern crate descriptor;
use std::time::Duration;
use std::thread::sleep;

use descriptor::*;
fn main() {
    describe("descriptor", |eg| {
        eg.it("1", || {
            panic!("Oh no");
        });
        eg.it("2", || {
            panic!("Oh no");
        });
        eg.it("3", || {
            panic!("Oh no");
        });

        eg.it("works", || {
        });
    });

    describe("descriptor2", |eg| {
        eg.it("17", || {
        });

        eg.it("does a lot of hard work", || {
            sleep(Duration::new(0, 500_000));
        });
    });

    descriptor_main();
}
