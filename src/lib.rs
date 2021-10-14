//! A Rust library to easily compare version numbers in any format, and test them against various comparison operators.
//!
//! Comparing version numbers is hard. Especially when version numbers get really complex,
//! or when their formatting differs.
//!
//! This library helps you to easily compare any kind of version number with minimal code.
//! Two version numbers can be compared to each other, to get a relevant comparison operator (`<`, `==`, `>`),
//! or version numbers can be tested against any given comparison operator.
//!
//! Along with version comparison, the library also features other useful tools.  
//! For example: version numbers can be parsed to inspect a version number by it's bare numeric or text based parts.
//!
//! Inspired by PHPs [version_compare()](http://php.net/manual/en/function.version-compare.php).
//!
//! ### Formats
//!
//! A list of version number examples that are parsed successfully:
//!
//! - `1`
//! - `3.10.4.1`
//! - `1.2.alpha`
//! - `1.2.dev.4`
//! - ` ` _(empty)_
//! - ` .   -32 . 1` _(undefined formats)_
//! - `MyApp 3.2.0 / build 0932` _(complex formats, not fully functional yet)_
//! - _Many more and support for custom formats to come..._
//!
//! ### Semver
//!
//! Version number formats like [_semver_](http://semver.org/) try to make version numbers consistent and manageable,
//! there are too many projects however that don't follow such format.
//!
//! `version-compare` makes working with them easy and supports semver formats out of the box with zero configuration.
//!
//! ## Features
//!
//! * Compare two version numbers, get: `<`, `==` or `>`.
//! * Compare two version numbers against a comparison operator
//! * Parse complex and undefined version number formats.
//! * Static, standalone methods to easily compare version strings
//!
//! The following features will be added in a later version:
//!
//! * Version manifest, to specify detailed version number constraints.
//! * Version ranges, and tests against them.
//! * Support for operators in version strings, [npm-style](https://docs.npmjs.com/misc/semver), and tests against them.
//! * Batch comparisons.
//!
//! ## Examples
//!
//! [example.rs:](examples/example.rs)
//! ```rust
//! use version_compare::{Cmp, Version};
//!
//! fn main() {
//!     // Define some version numbers
//!     let a = "1.2";
//!     let b = "1.5.1";
//!
//!     // The following comparison operators are used:
//!     // - Cmp::Eq -> Equal
//!     // - Cmp::Ne -> Not equal
//!     // - Cmp::Lt -> Less than
//!     // - Cmp::Le -> Less than or equal
//!     // - Cmp::Ge -> Greater than or equal
//!     // - Cmp::Gt -> Greater than
//!
//!     // Easily compare version strings
//!     assert_eq!(version_compare::compare(a, b).unwrap(), Cmp::Lt);
//!     assert_eq!(version_compare::compare_to(a, b, Cmp::Le).unwrap(), true);
//!     assert_eq!(version_compare::compare_to(a, b, Cmp::Gt).unwrap(), false);
//!
//!     // Version string parsing
//!     let a = Version::from(a).unwrap();
//!     let b = Version::from(b).unwrap();
//!
//!     // Directly compare parsed versions
//!     assert_eq!(a < b, true);
//!     assert_eq!(a <= b, true);
//!     assert_eq!(a > b, false);
//!     assert_eq!(a != b, true);
//!     assert_eq!(a.compare(&b), Cmp::Lt);
//!     assert_eq!(b.compare(&a), Cmp::Gt);
//!     assert_eq!(a.compare_to(&b, Cmp::Lt), true);
//!
//!     // Match
//!     match a.compare(b) {
//!         Cmp::Lt => println!("Version a is less than b"),
//!         Cmp::Eq => println!("Version a is equal to b"),
//!         Cmp::Gt => println!("Version a is greater than b"),
//!         _ => unreachable!(),
//!     }
//! }
//! ```
//!
//! Check out the [examples](https://github.com/timvisee/version-compare/tree/master/examples) directory for more.
//!
//! _[View complete README](https://github.com/timvisee/version-compare/blob/master/README.md)_

mod cmp;
mod compare;
mod manifest;
mod part;
mod version;

#[cfg(test)]
mod test;

// Re-exports
pub use crate::cmp::Cmp;
pub use crate::compare::{compare, compare_to};
pub use crate::manifest::Manifest;
pub use crate::part::Part;
pub use crate::version::Version;
