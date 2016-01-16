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

        eg.it("does a lot of hard work", || {
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;
            while i < 1000 {
                while j < 10000 {
                    j += 1;
                    k = 0;
                }
                i += 1;
                j = 0;
                k = 0;
            }
        });
    });

    descriptor_main();
}
