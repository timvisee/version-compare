//! Version module, which provides the `Version` struct as parsed version representation.
//!
//! Version numbers in the form of a string are parsed to a `Version` first, before any comparison
//! is made. This struct provides many methods and features for easy comparison, probing and other
//! things.

use std::cmp::Ordering;
use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;

use crate::comp_op::CompOp;
use crate::version_part::{VersionPart, ProvideEmptyImpl};
use crate::parsers::default::default_parser;

/// Version struct, which is a representation for a parsed version string.
///
/// A version in string format can be parsed using methods like `Version::from("1.2.3");`.
/// These methods return a `Result` holding the parsed version or an error on failure.
///
/// The original version string is stored in the struct, and can be accessed using the
/// `version.as_str()` method. Note, that when the version wasn't parsed from a string
/// representation, the returned value is generated.
///
/// The struct provides many methods for comparison and probing.
pub struct Version<'a> {
    version: &'a str,
    parts: Vec<VersionPart<'a>>,
}

impl<'a> Version<'a> {
    /// Create a `Version` instance from a version string.
    ///
    /// The version string should be passed to the `version` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{CompOp, Version};
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.compare(&Version::from("1.2.3").unwrap()), CompOp::Eq);
    /// ```
    pub fn from(version: &'a str) -> Option<Version> {
        Version::parse(version, &default_parser)
    }

    /// Create a `Version` instance from a version string with the given `parser` function.
    ///
    /// The version string should be passed to the `version` parameter.  Additional parsers
    /// are in the "parsers" module.  This is the primary means of customizing behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{CompOp, Version, default_parser};
    ///
    /// let ver = Version::parse("1.2.3", &default_parser).unwrap();
    ///
    /// assert_eq!(ver.compare(&Version::from("1.2.3").unwrap()), CompOp::Eq);
    /// ```
    pub fn parse(version: &'a str, parser: &dyn Fn(&'a str) -> Option<Vec<VersionPart<'a>>>) -> Option<Self> {
        let parts: Option<Vec<VersionPart<'a>>> = parser(version);

        if parts.is_none() {
            return None;
        }

        Some(Self {
            version,
            parts: parts.unwrap(),
        })
    }

    /// Get the original version string.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Version;
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.as_str(), "1.2.3");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.version
    }

    /// Get a specific version part by it's `index`.
    /// An error is returned if the given index is out of bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Version, VersionPart};
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.part(0), Ok(&VersionPart::Integer(1)));
    /// assert_eq!(ver.part(1), Ok(&VersionPart::Integer(2)));
    /// assert_eq!(ver.part(2), Ok(&VersionPart::Integer(3)));
    /// ```
    pub fn part(&self, index: usize) -> Result<&VersionPart<'a>, ()> {
        // Make sure the index is in-bound
        if index >= self.parts.len() {
            return Err(());
        }

        // Return the requested part
        Ok(&self.parts[index])
    }

    /// Get a vector of all version parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Version, VersionPart};
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.parts(), &vec![
    ///     VersionPart::Integer(1),
    ///     VersionPart::Integer(2),
    ///     VersionPart::Integer(3)
    /// ]);
    /// ```
    pub fn parts(&self) -> &Vec<VersionPart> {
        &self.parts
    }

    /// Get the number of parts in this version string.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Version;
    ///
    /// let ver_a = Version::from("1.2.3").unwrap();
    /// let ver_b = Version::from("1.2.3.4").unwrap();
    ///
    /// assert_eq!(ver_a.part_count(), 3);
    /// assert_eq!(ver_b.part_count(), 4);
    /// ```
    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    /// Compare this version to the given `other` version.
    ///
    /// This method returns one of the following comparison operators:
    ///
    /// * `Lt`
    /// * `Eq`
    /// * `Gt`
    ///
    /// Other comparison operators can be used when comparing, but aren't returned by this method.
    ///
    /// # Examples:
    ///
    /// ```
    /// use version_compare::{CompOp, Version};
    ///
    /// assert_eq!(Version::from("1.2").unwrap().compare(&Version::from("1.3.2").unwrap()), CompOp::Lt);
    /// assert_eq!(Version::from("1.9").unwrap().compare(&Version::from("1.9").unwrap()), CompOp::Eq);
    /// assert_eq!(Version::from("0.3.0.0").unwrap().compare(&Version::from("0.3").unwrap()), CompOp::Eq);
    /// assert_eq!(Version::from("2").unwrap().compare(&Version::from("1.7.3").unwrap()), CompOp::Gt);
    /// ```
    pub fn compare(&self, other: &'a Version) -> CompOp {
        // Compare the versions with their peekable iterators
        Self::compare_iter(self.parts.iter().peekable(), other.parts.iter().peekable())
    }

    /// Compare this version to the given `other` version,
    /// and check whether the given comparison operator is valid.
    ///
    /// All comparison operators can be used.
    ///
    /// # Examples:
    ///
    /// ```
    /// use version_compare::{CompOp, Version};
    ///
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.3.2").unwrap(), &CompOp::Lt));
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.3.2").unwrap(), &CompOp::Le));
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.2").unwrap(), &CompOp::Eq));
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.2").unwrap(), &CompOp::Le));
    /// ```
    pub fn compare_to(&self, other: &Version, operator: &CompOp) -> bool {
        // Get the comparison result
        let result = self.compare(&other);

        // Match the result against the given operator
        match result {
            CompOp::Eq => match operator {
                &CompOp::Eq | &CompOp::Le | &CompOp::Ge => true,
                _ => false,
            },
            CompOp::Lt => match operator {
                &CompOp::Ne | &CompOp::Lt | &CompOp::Le => true,
                _ => false,
            },
            CompOp::Gt => match operator {
                &CompOp::Ne | &CompOp::Gt | &CompOp::Ge => true,
                _ => false,
            },
            _ => unreachable!(),
        }
    }

    /// Compare two version numbers based on the iterators of their version parts.
    ///
    /// This method returns one of the following comparison operators:
    ///
    /// * `Lt`
    /// * `Eq`
    /// * `Gt`
    ///
    /// Other comparison operators can be used when comparing, but aren't returned by this method.
    fn compare_iter(
        mut iter: Peekable<Iter<VersionPart>>,
        mut other_iter: Peekable<Iter<VersionPart>>,
    ) -> CompOp {
        // Iterate over the iterator, without consuming it
        loop {
            let i1 = iter.next();
            let i2 = other_iter.next();
            let _cmp = match (i1, i2) {
                (Some(i), None) => match i.partial_cmp(&i.get_empty()) {
                    Some(Ordering::Less) => return CompOp::Lt,
                    Some(Ordering::Greater) => return CompOp::Gt,
                    Some(Ordering::Equal) => return CompOp::Eq,
                    _ => panic!()
                },
                (None, Some(j)) => match j.get_empty().partial_cmp(j) {
                    Some(Ordering::Less) => return CompOp::Lt,
                    Some(Ordering::Greater) => return CompOp::Gt,
                    Some(Ordering::Equal) => return CompOp::Eq,
                    _ => panic!()
                },
                (Some(i), Some(j)) => match i.partial_cmp(j) {
                    Some(Ordering::Greater) => return CompOp::Gt,
                    Some(Ordering::Less) => return CompOp::Lt,
                    // This is the only loop branch that continues
                    Some(Ordering::Equal) => Ordering::Equal,
                    _ => panic!()
                },
                // both versions are the same length and are equal for all values
                (None, None) => return CompOp::Eq
            };
        }
    }
}


impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

// Show just the version component parts as debug output
impl<'a> fmt::Debug for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.parts)
        } else {
            write!(f, "{:?}", self.parts)
        }
    }
}

/// Implement the partial ordering trait for the version struct, to easily allow version comparison.
impl<'a> PartialOrd for Version<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other).ord()
    }
}

/// Implement the partial equality trait for the version struct, to easily allow version comparison.
impl<'a> PartialEq for Version<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.compare_to(other, &CompOp::Eq)
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use crate::comp_op::CompOp;
    use crate::test::test_version::{TEST_VERSIONS, TEST_VERSIONS_ERROR};
    use crate::test::test_version_set::TEST_VERSION_SETS;
    // use crate::version_part::VersionPart;

    use super::Version;

    #[test]
    // TODO: This doesn't really test whether this method fully works
    fn from() {
        // Test whether parsing works for each test version
        for version in TEST_VERSIONS {
            assert!(Version::from(&version.0).is_some());
        }

        // Test whether parsing works for each test invalid version
        for version in TEST_VERSIONS_ERROR {
            assert!(Version::from(&version.0).is_none());
        }
    }

    #[test]
    fn as_str() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // The input version string must be the same as the returned string
            assert_eq!(Version::from(&version.0).unwrap().as_str(), version.0);
        }
    }

    #[test]
    fn part() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // Create a version object
            let ver = Version::from(&version.0).unwrap();

            // Loop through each part
            for i in 0..version.1 {
                assert_eq!(ver.part(i), Ok(&ver.parts[i]));
            }

            // A value outside the range must return an error
            assert!(ver.part(version.1).is_err());
        }
    }

    #[test]
    fn parts() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // The number of parts must match
            assert_eq!(Version::from(&version.0).unwrap().parts().len(), version.1);
        }
    }

    #[test]
    fn part_count() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // The number of parts must match the metadata
            assert_eq!(Version::from(&version.0).unwrap().part_count(), version.1);
        }
    }

    #[test]
    fn compare() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Get both versions
            let version_a = Version::from(&entry.0).unwrap();
            let version_b = Version::from(&entry.1).unwrap();

            // Compare them
            assert_eq!(
                version_a.compare(&version_b),
                entry.2.clone(),
                "Testing that {} is {} {}",
                &entry.0,
                &entry.2.sign(),
                &entry.1
            );
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Get both versions
            let version_a = Version::from(&entry.0).unwrap();
            let version_b = Version::from(&entry.1).unwrap();

            // Test
            assert!(version_a.compare_to(&version_b, &entry.2));

            // Make sure the inverse operator is not correct
            assert_eq!(version_a.compare_to(&version_b, &entry.2.invert()), false);
        }

        // Assert an exceptional case, compare to not equal
        assert!(Version::from("1.2")
            .unwrap()
            .compare_to(&Version::from("1.2.3").unwrap(), &CompOp::Ne,));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Version::from("1.2.3").unwrap()), "1.2.3");
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Version::from("1.2.3").unwrap()),
            "[Integer(1), Integer(2), Integer(3)]",
        );
        assert_eq!(
            format!("{:#?}", Version::from("1.2.3").unwrap()),
            "[\n    Integer(\n        1,\n    ),\n    Integer(\n        2,\n    ),\n    Integer(\n        3,\n    ),\n]",
        );
    }

    #[test]
    fn partial_cmp() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Get both versions
            let version_a = Version::from(&entry.0).unwrap();
            let version_b = Version::from(&entry.1).unwrap();

            // Compare and assert
            match entry.2 {
                CompOp::Eq => assert!(version_a == version_b),
                CompOp::Lt => assert!(version_a < version_b),
                CompOp::Gt => assert!(version_a > version_b),
                _ => {}
            }
        }
    }

    #[test]
    fn partial_eq() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Skip entries that are less or equal, or greater or equal
            match entry.2 {
                CompOp::Le | CompOp::Ge => continue,
                _ => {}
            }

            // Get both versions
            let version_a = Version::from(&entry.0).unwrap();
            let version_b = Version::from(&entry.1).unwrap();

            // Determine what the result should be
            let result = match entry.2 {
                CompOp::Eq => true,
                _ => false,
            };

            // Test
            assert_eq!(version_a == version_b, result);
        }

        // Assert an exceptional case, compare to not equal
        assert!(Version::from("1.2").unwrap() != Version::from("1.2.3").unwrap());
    }
}
