extern crate rand as r;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
mod compound_datatypes;
mod functions;
mod ownership;
mod printing_slice;

fn main() {
    printing_slice::main();
}

fn find_space(my_str: &str) -> usize {
    let bytes = my_str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    my_str.len()
}

fn find_max(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &value in slice.iter().skip(1) {
        if value > max {
            max = value;
        }
    }
    max
}
