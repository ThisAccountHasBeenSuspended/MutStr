/*
 * January 8th, 2024
 * Intel Core i3-13100
 * 2x 8GB DDR4-3600 CL17-21-21
 */

#![feature(test)]

extern crate test;

use mutstr::mutstr;
use test::Bencher;

// 29 ns/iter (+/- 1)
#[bench]
fn create_box(b: &mut Bencher) {
    b.iter(|| Box::<str>::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo"));
}

// 29 ns/iter (+/- 1)
#[bench]
fn create_string(b: &mut Bencher) {
    b.iter(|| String::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo"));
}

// 29 ns/iter (+/- 0)
#[bench]
fn create_mutstr(b: &mut Bencher) {
    b.iter(|| mutstr::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo"));
}

// It is impossible to replace the data, so we have to create a new `Box<str>`.
// 71 ns/iter (+/- 2)
#[bench]
fn replace_box_data(b: &mut Bencher) {
    let mut result = Box::<str>::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo");
    let mut number = 0;
    b.iter(|| {
        result = Box::<str>::from(format!("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo{}", number).as_str());
        number += 1;
    });
}

// 51 ns/iter (+/- 3)
#[bench]
fn replace_string_data(b: &mut Bencher) {
    let mut result = String::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo");
    let mut number = 0;
    b.iter(|| {
        result.replace_range(.., format!("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo{}", number).as_str());
        number += 1;
    });
}

// 46 ns/iter (+/- 3)
#[bench]
fn replace_mutstr_data(b: &mut Bencher) {
    let mut result = mutstr::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo");
    let mut number = 0;
    b.iter(|| {
        result.replace_with(format!("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo{}", number).as_str());
        number += 1;
    });
}
