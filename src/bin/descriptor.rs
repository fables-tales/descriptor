extern crate descriptor;

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
            println!("this one ran too");
        });
    });

    descriptor_main();
}
