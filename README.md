[![Build Status on Travis CI](https://travis-ci.org/timvisee/version-compare.svg?branch=master)](https://travis-ci.org/timvisee/version-compare)
[![Library on crates.io](https://img.shields.io/crates/v/version-compare.svg)](https://crates.io/crates/version-compare)
[![Download statistics on crates.io](https://img.shields.io/crates/d/version-compare.svg)](https://crates.io/crates/version-compare)
[![Coverage Status](https://coveralls.io/repos/github/timvisee/version-compare/badge.svg?branch=master)](https://coveralls.io/github/timvisee/version-compare?branch=master)
[![Library on crates.io](https://img.shields.io/crates/l/version-compare.svg)](https://crates.io/crates/version-compare)

# Rust library: version-compare
> A Rust library to easily compare version numbers, and test them against various comparison operators.

[Documentation](https://timvisee.github.io/version-compare)

Comparing version numbers is hard. Especially when version numbers get really complex,
or when their formatting differs. 

This library helps you to easily compare any kind of version number with a single code-statement.
Two version numbers can be compared to each other, to get a relevant comparison operator (`<`, `==`, `>`),
or version numbers can be tested against any given comparison operator.

Along with version comparison, the library also features other useful tools.  
For example: version numbers can also be parsed to inspect a version number by it's bare _parts_ if desired.

Inspired by PHP's [version_compare()](http://php.net/manual/en/function.version-compare.php).

Note: This library is still a work in progress.
See the list below for a list of currently available and future features.

## Features
* Compare two version numbers, get: `<`, `==` or `>`.
* Compare two version numbers against any comparison operator.
* Parse complex version numbers.
* Static, single-statement methods available.

The following features will be added in a later version:
* Version manifest, to specify detailed version number constraints.
* Batch comparisons.

## Example
This library is very easy to use. Here's a basic usage example:

Cargo.toml:
```toml
[dependencies]
version-compare = "0.0.3"
```

main.rs:
```rust
use version_compare::VersionCompare;
use version_compare::version::Version;
use version_compare::comp_op::CompOp;

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
}
```

## Builds
This library is automatically build and tested for each commit using CI services.

|Service|Branch|Build Status| |
|---:|:---|:---:|:---|
|Travis CI|master|[![Build status on Travis CI on master](https://travis-ci.org/timvisee/version-compare.svg?branch=master)](https://travis-ci.org/timvisee/version-compare)|[View Status](https://travis-ci.org/timvisee/version-compare)|
|Travis CI|last commit|[![Build status on Travis CI for last commit](https://travis-ci.org/timvisee/version-compare.svg)](https://travis-ci.org/timvisee/version-compare)|[View Status](https://travis-ci.org/timvisee/version-compare)|

## License
This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.