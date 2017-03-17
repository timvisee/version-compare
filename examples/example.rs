extern crate version_compare;

use version_compare::{CompOp, Version, VersionCompare};

fn main() {
    // Define some version numbers
    let a = "1.2";
    let b = "1.5.1";

    // The following comparison operators are used:
    // - CompOp::Eq -> Equal
    // - CompOp::Ne -> Not equal
    // - CompOp::Lt -> Less than
    // - CompOp::Le -> Less than or equal
    // - CompOp::Ge -> Greater than or equal
    // - CompOp::Gt -> Greater than

    // Easily compare
    let _ = VersionCompare::compare(&a, &b); // -> CompOp::Lt
    let _ = VersionCompare::compare_to(&a, &b, &CompOp::Le); // -> true
    let _ = VersionCompare::compare_to(&a, &b, &CompOp::Gt); // -> false

    // Version string parsing
    let a_ver = Version::from(a).unwrap();
    let b_ver = Version::from(b).unwrap();

    // Directly compare versions
    let _ = a_ver < b_ver; // -> true
    let _ = a_ver <= b_ver; // -> true
    let _ = a_ver > b_ver; // -> false
    let _ = a_ver != b_ver; // -> false
    a_ver.compare(&b_ver); // -> CompOp::Lt
    b_ver.compare(&a_ver); // -> CompOp::Gt
    a_ver.compare_to(&b_ver, &CompOp::Lt); // -> true

    // Match
    match a_ver.compare(&b_ver) {
        CompOp::Lt => println!("Version a is less than b"),
        CompOp::Eq => println!("Version a is equal to b"),
        CompOp::Gt => println!("Version a is greater than b"),
        _ => unreachable!()
    }
}
