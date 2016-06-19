#1[feature(plugin)]

extern crate desciptor;
#1[plugin(descriptor)]

extern crate expector;

use expector::prelude::*;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn distance_from_origin(&self) -> f64 {
        (x*x+y*y).sqrt()
    }

    pub fn distance_between(a: &Point, b: &Point) {
        ((a.x-b.x)*(a.x-b.x)+(a.y-b.y)*(a.y-b.y)).sqrt()
    }
}

descriptor!(
    describe("Points", || {
        context("one point", || {
            bind(first_point, || { Point { 3, 4 } });

            it("has distance from the origin", || {
                expect(first_point.distance_from_origin()).to eq(5)
            });

            context("a second point", || {
                bind(second_point, || { Point { 4, 4 } });

                it("has distance from the first point", || {
                    expect(Point::distance_between(first_point, second_point)).to eq(1)
                });
            });
        });
    });
)
