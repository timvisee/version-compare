//! The most minimal usage example of the version-compare library.
//!
//! This example compares two given version numbers, and matches the comparison result.
//!
//! You can run this example file by using the command `cargo run --example minimal`.

use version_compare::{compare, Cmp};

fn main() {
    let a = "1.3";
    let b = "1.2.4";

    match compare(a, b).unwrap() {
        Cmp::Lt => println!("Version a is less than b"),
        Cmp::Eq => println!("Version a is equal to b"),
        Cmp::Gt => println!("Version a is greater than b"),
        _ => unreachable!(),
    }
}
