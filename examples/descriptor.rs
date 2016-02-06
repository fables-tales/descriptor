extern crate descriptor;
extern crate expector;

use std::time::Duration;
use std::thread::sleep;

use descriptor::*;
use expector::*;

fn main() {
    describe("example group 1", |eg| {
        eg.it("1", || {
            expect(1).to(eq(2));
        });
        eg.it("2", || {
            expect("abc").to(eq("def"));
        });
        eg.it("3", || {
            expect(None).to(eq(Some(3)));
        });

        eg.it("works", || {
        });
    });

    describe("example group 2", |eg| {
        eg.it("17", || {
        });

        eg.it("does a lot of hard work", || {
            sleep(Duration::new(3, 0));
        });
    });

    descriptor_main();
}
