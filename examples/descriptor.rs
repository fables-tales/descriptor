#[macro_use]
extern crate descriptor;
extern crate expector;

use std::time::Duration;
use std::thread::sleep;

use descriptor::*;
use expector::*;

fn main() {
    describe("example group 1", source_location!(), |eg| {
        eg.it("1", source_location!(), || {
            expect(1).to(eq(2));
        });
        eg.it("2", source_location!(), || {
            expect("abc").to(eq("def"));
        });
        eg.it("3", source_location!(), || {
            expect(None).to(eq(Some(3)));
        });

        eg.it("works", source_location!(), || {
        });
    });

    describe("example group 2", source_location!(), |eg| {
        eg.it("17", source_location!(), || {
        });

        eg.it("does a lot of hard work", source_location!(), || {
            sleep(Duration::new(3, 0));
        });
    });

    descriptor_main();
}
