/*
 * January 13th, 2024
 * Intel Core i3-13100
 * 2x 8GB DDR4-3600 CL17-21-21
 */

#![feature(test)]

extern crate test;

use mutstr::mutstr;
use test::Bencher;

// 28 ns/iter (+/- 3)
#[bench]
fn create_box(b: &mut Bencher) {
    b.iter(|| Box::<str>::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo"));
}

// 28 ns/iter (+/- 1)
#[bench]
fn create_string(b: &mut Bencher) {
    b.iter(|| String::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo"));
}

// 28 ns/iter (+/- 0)
#[bench]
fn create_mutstr(b: &mut Bencher) {
    b.iter(|| mutstr::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo"));
}

// 104 ns/iter (+/- 3)
#[bench]
fn replace_box_data(b: &mut Bencher) {
    let mut result = Box::<str>::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo");
    let mut number = 0;
    b.iter(|| {
        // It is impossible to replace the data, so we have to create a new `Box<str>`.
        result = Box::<str>::from(format!("ƒoo{}", number).as_str());
        number += 1;
    });
}

// 85 ns/iter (+/- 7)
#[bench]
fn replace_string_data(b: &mut Bencher) {
    let mut result = String::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo");
    let mut number = 0;
    b.iter(|| {
        result.replace_range(.., format!("ƒoo{}", number).as_str());
        number += 1;
    });
}

// 82 ns/iter (+/- 1)
#[bench]
fn replace_mutstr_data(b: &mut Bencher) {
    let mut result = mutstr::from("ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo");
    let mut number = 0;
    b.iter(|| {
        result.replace_with(format!("ƒoo{}", number).as_str());
        number += 1;
    });
}
