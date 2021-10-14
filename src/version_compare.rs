//! Version compare module, with useful static comparison methods.
//!
//! This module provides the `VersionCompare` struct, which provides many static functions, that are
//! useful for version comparison.

use crate::version::Version;
use crate::Cmp;

/// The main library structure, which provides various static methods for easy version comparison.
///
/// This structure uses static methods only, and doesn't need to be constructed.
pub struct VersionCompare {}

impl VersionCompare {
    /// Compare two version number strings to each other.
    /// This compares version `a` to version `b`, and returns whether version `a` is greater, less
    /// or equal to version `b`.
    ///
    /// The two given version numbers must be valid, or an error will be returned.
    ///
    /// One of the following ok results may be returned:
    ///
    /// * `Cmp::Eq`
    /// * `Cmp::Lt`
    /// * `Cmp::Gt`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Cmp, VersionCompare};
    ///
    /// // Compare version numbers
    /// assert_eq!(VersionCompare::compare("1.2.3", "1.2.3"), Ok(Cmp::Eq));
    /// assert_eq!(VersionCompare::compare("1.2.3", "1.2.4"), Ok(Cmp::Lt));
    /// assert_eq!(VersionCompare::compare("1", "0.1"), Ok(Cmp::Gt));
    /// ```
    pub fn compare(a: &str, b: &str) -> Result<Cmp, ()> {
        // Create version instances
        let a_ver = Version::from(a);
        let b_ver = Version::from(b);

        // Both version numbers must have been parsed
        if a_ver.is_none() || b_ver.is_none() {
            return Err(());
        }

        // Compare and return the result
        Ok(a_ver.unwrap().compare(&b_ver.unwrap()))
    }

    /// Compare two version number strings to each other and check whether the given comparison
    /// `operator` is valid.
    ///
    /// The two given version numbers must be valid, or an error will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Cmp, VersionCompare};
    ///
    /// // Compare version numbers
    /// assert!(VersionCompare::compare_to("1.2.3", "1.2.3", Cmp::Eq).unwrap());
    /// assert!(VersionCompare::compare_to("1.2.3", "1.2.3", Cmp::Le).unwrap());
    /// assert!(VersionCompare::compare_to("1.2.3", "1.2.4", Cmp::Lt).unwrap());
    /// assert!(VersionCompare::compare_to("1", "0.1", Cmp::Gt).unwrap());
    /// assert!(VersionCompare::compare_to("1", "0.1", Cmp::Ge).unwrap());
    /// ```
    pub fn compare_to(a: &str, b: &str, operator: Cmp) -> Result<bool, ()> {
        // Create version instances
        let a_ver = Version::from(a);
        let b_ver = Version::from(b);

        // Both version numbers must have been parsed
        if a_ver.is_none() || b_ver.is_none() {
            return Err(());
        }

        // Compare and return the result
        Ok(a_ver.unwrap().compare_to(&b_ver.unwrap(), operator))
    }
}

// TODO: keep this cfg check? used in other places as well?
#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use crate::test::test_version_set::{TEST_VERSION_SETS, TEST_VERSION_SETS_ERROR};
    use crate::Cmp;

    use super::VersionCompare;

    #[test]
    fn compare() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            assert_eq!(
                VersionCompare::compare(&entry.0, &entry.1),
                Ok(entry.2.clone()),
                "Testing that {} is {} {}",
                &entry.0,
                &entry.2.sign(),
                &entry.1,
            );
        }

        // Compare each error version in the version set
        for entry in TEST_VERSION_SETS_ERROR {
            let result = VersionCompare::compare(&entry.0, &entry.1);

            if result.is_ok() {
                assert!(result != Ok(entry.2.clone()));
            }
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Test
            assert!(VersionCompare::compare_to(&entry.0, &entry.1, entry.2).unwrap());

            // Make sure the inverse operator is not correct
            assert_eq!(
                VersionCompare::compare_to(&entry.0, &entry.1, entry.2.invert()).unwrap(),
                false
            );
        }

        // Compare each error version in the version set
        for entry in TEST_VERSION_SETS_ERROR {
            let result = VersionCompare::compare_to(&entry.0, &entry.1, entry.2);

            if result.is_ok() {
                assert!(!result.unwrap())
            }
        }

        // Assert an exceptional case, compare to not equal
        assert!(VersionCompare::compare_to("1.2.3", "1.2", Cmp::Ne).unwrap());
    }
}
