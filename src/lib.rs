//! A rust library to easily compare version numbers,
//! and test them against various comparison operators.
//!
//! Comparing version numbers is hard. Especially when version numbers get really complex, or when their formatting differs.
//!
//! This library helps you to easily compare any kind of version number with a single code-statement. Two version numbers can be compared to each other, to get a relevant comparison operator (<, ==, >), or version numbers can be tested against any given comparison operator.
//!
//! Along with version comparison, the library also features other useful tools.
//! For example: version numbers can be parsed to inspect a version number by it's bare numeric or text based parts.
//!
//! Inspired by PHPs [version_compare()](http://php.net/manual/en/function.version-compare.php).
//!
//! ## Features
//! * Compare two version numbers, get: `<`, `==` or `>`.
//! * Compare two version numbers against any comparison operator.
//! * Parse complex version numbers.
//! * Static, single-statement methods available.
//!
//! The following features will be added in a later version:
//! * Version manifest, to specify detailed version number constraints.
//! * Batch comparisons.
//!
//! ## Examples
//! Check out the [examples](https://github.com/timvisee/version-compare/tree/master/examples) directory for all available examples.
//!
//!
//! _[View complete README](https://github.com/timvisee/version-compare/blob/master/README.md)_

pub mod comp_op;
pub mod version;
pub mod version_compare;
pub mod version_manifest;
pub mod version_part;

#[cfg(test)]
mod test;

