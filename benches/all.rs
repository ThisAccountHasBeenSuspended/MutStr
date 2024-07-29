/*
 * July 29th, 2024
 * Intel Core i5-12400f
 * 2x 8GB DDR4-3600 CL17-21-21
 */

#![feature(test)]

extern crate test;

use mutstr::mutstr;
use test::Bencher;

const TEXT: &str = "ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo ƒoo";

#[bench]
fn create_box(b: &mut Bencher) {
    b.iter(|| Box::<str>::from(TEXT));
}

#[bench]
fn create_string(b: &mut Bencher) {
    b.iter(|| String::from(TEXT));
}

#[bench]
fn create_mutstr(b: &mut Bencher) {
    b.iter(|| mutstr::from(TEXT));
}

#[bench]
fn replace_box_data(b: &mut Bencher) {
    let mut result = Box::<str>::from("");
    let mut number = 0;
    b.iter(|| {
        // It is impossible to replace the data, so we have to create a new `Box<str>`.
        result = Box::<str>::from(format!("{TEXT}{number}").as_str());
        number += 1;
    });
}

#[bench]
fn replace_string_data(b: &mut Bencher) {
    let mut result = String::from("");
    let mut number = 0;
    b.iter(|| {
        result.replace_range(.., format!("{TEXT}{number}").as_str());
        number += 1;
    });
}

#[bench]
fn replace_mutstr_data(b: &mut Bencher) {
    let mut result = mutstr::from("");
    let mut number = 0;
    b.iter(|| {
        result.replace_with(format!("{TEXT}{number}").as_str());
        number += 1;
    });
}
