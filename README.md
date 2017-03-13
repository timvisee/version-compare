[![Build Status on Travis CI](https://travis-ci.org/timvisee/version-compare.svg?branch=master)](https://travis-ci.org/timvisee/version-compare)

# Rust library: version-compare
> A Rust library to easily compare version numbers, and test them against various comparison operators.

Comparing version numbers is hard. Especially when version numbers get really complex,
or when their formatting differs. 

This library helps you to easily compare any kind of version number with a single code-statement.
Version numbers can also be parsed to inspect a version number by it's _parts_ if desired.

Note: This library is still a work in progress. See the list below for a list of currently available and future features.

## Features
* Compare two version numbers, get: `<`, `==` or `>`.
* Compare two version numbers against any comparison operator.
* Parse complex version numbers.
* Static, single-statement methods available.

The following features will be added in a later version:
* Version manifest, to specify detailed version number constraints.
* Batch comparisons.

## Usage
This library is very easy to use. Here's a basic usage example:

```rust
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
VersionCompare::compare(&a, &b); // -> CompOp::Lt
VersionCompare::compare_to(&a, &b, &CompOp::Le); // -> true
VersionCompare::compare_to(&a, &b, &CompOp::Gt); // -> false

// Version string parsing
let a_ver = Version::from(a).unwrap();
let b_ver = Version::from(b).unwrap();

// Directly compare versions
a_ver.compare(&b_ver); // -> CompOp::Lt
b_ver.compare(&a_ver); // -> CompOp::Gt
a_ver.compare_to(&b_ver, &CompOp::LT); // -> true

// Match
match a_ver.compare(&b_ver) {
    CompOp::Lt => println!("Version a is less than b"),
    CompOp::Eq => println!("Version a is equal to b"),
    CompOp::Gt => println!("Version a is greater than b")
}
```

## License
This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.