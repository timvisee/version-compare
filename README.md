[![Build Status on Travis CI](https://travis-ci.org/timvisee/version-compare.svg?branch=master)](https://travis-ci.org/timvisee/version-compare)

# version-compare (Rust library)
An easy to use library for rust to compare version strings.

This library is still a work in progress.

## Usage
This shows a possible usage representation of the library. This might change, as the library is still a work in progress.

```Rust
// Define some version numbers
let a = "1.2";
let b = "1.5.1";

// The following comparison operators are used:
// - CompOp::EQ -> Equal
// - CompOp::NE -> Not equal
// - CompOp::LT -> Less than
// - CompOp::LE -> Less than or equal to
// - CompOp::GE -> Greater than or equal to
// - CompOp::GT -> Greater than

// Easily compare
VersionCompare::compare(&a, &b); // -> CompOp::LT
VersionCompare::compare_to(&a, &b, &CompOp::LE); // -> true
VersionCompare::compare_to(&a, &b, &CompOp::GT); // -> false

// Version string parsing
let a_ver = Version::from(a).unwrap();
let b_ver = Version::from(b).unwrap();

// Directly compare versions
a_ver.compare(&b_ver); // -> CompOp::LT
b_ver.compare(&a_ver); // -> CompOp::GT
a_ver.compare_to(&b_ver, &CompOp::LT); // -> true

// Match
match a_ver.compare(&b_ver) {
    CompOp::LT => println!("Version a is less than b"),
    CompOp::EQ => println!("Version a is equal to b"),
    CompOp::GT => println!("Version a is greater than b")
}
```

## License
This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.