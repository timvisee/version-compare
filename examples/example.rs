//! Usage examples of the version-compare library.
//!
//! This file shows various ways this library supports for comparing version numbers,
//! and it shows various ways of implementing it in code logic such as with a `match` statement.
//!
//! The `assert_eq!(...)` macros are used to assert the returned value by a given statement.
//!
//! You can run this example file by using the command `cargo run --example example`.

extern crate version_compare;

use version_compare::{Cmp, Version, VersionCompare};

fn main() {
    // Define some version numbers
    let a = "1.2";
    let b = "1.5.1";

    // The following comparison operators are used:
    // - Cmp::Eq -> Equal
    // - Cmp::Ne -> Not equal
    // - Cmp::Lt -> Less than
    // - Cmp::Le -> Less than or equal
    // - Cmp::Ge -> Greater than or equal
    // - Cmp::Gt -> Greater than

    // Easily compare version strings
    assert_eq!(VersionCompare::compare(&a, &b).unwrap(), Cmp::Lt);
    assert_eq!(VersionCompare::compare_to(&a, &b, Cmp::Le).unwrap(), true);
    assert_eq!(VersionCompare::compare_to(&a, &b, Cmp::Gt).unwrap(), false);

    // Version string parsing
    let a_ver = Version::from(a).unwrap();
    let b_ver = Version::from(b).unwrap();

    // Directly compare parsed versions
    assert_eq!(a_ver < b_ver, true);
    assert_eq!(a_ver <= b_ver, true);
    assert_eq!(a_ver > b_ver, false);
    assert_eq!(a_ver != b_ver, true);
    assert_eq!(a_ver.compare(&b_ver), Cmp::Lt);
    assert_eq!(b_ver.compare(&a_ver), Cmp::Gt);
    assert_eq!(a_ver.compare_to(&b_ver, Cmp::Lt), true);

    // Match
    match a_ver.compare(&b_ver) {
        Cmp::Lt => println!("Version a is less than b"),
        Cmp::Eq => println!("Version a is equal to b"),
        Cmp::Gt => println!("Version a is greater than b"),
        _ => unreachable!(),
    }
}
