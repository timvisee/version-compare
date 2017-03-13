pub mod comp_op;
pub mod version;
pub mod version_part;
#[cfg(test)]
mod test;

use comp_op::CompOp;
use version::Version;

/// Version compare structure.
pub struct VersionCompare { }

/// Version compare implementation.
impl VersionCompare {

    /// Create a new version compare instance.
    pub fn new() -> Self {
        VersionCompare { }
    }

    /// Compare two version number strings to each other.
    /// This compares version `a` to version `b`, and returns whether version `a` is greater, less
    /// or equal to version `b`.
    ///
    /// The two given version numbers must be valid, or an error will be returned.
    ///
    /// One of the following ok results may be returned:
    /// - CompOp::EQ
    /// - CompOp::LT
    /// - CompOp::GT
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::VersionCompare;
    /// use version_compare::comp_op::CompOp;
    ///
    /// let version_compare = VersionCompare::new();
    ///
    /// // Compare version numbers
    /// assert_eq!(version_compare.compare("1.2.3", "1.2.3"), Ok(CompOp::EQ));
    /// assert_eq!(version_compare.compare("1.2.3", "1.2.4"), Ok(CompOp::LT));
    /// assert_eq!(version_compare.compare("1", "0.1"), Ok(CompOp::GT));
    /// ```
    pub fn compare(&self, a: &str, b: &str) -> Result<CompOp, ()> {
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
    /// use version_compare::VersionCompare;
    /// use version_compare::comp_op::CompOp;
    ///
    /// let version_compare = VersionCompare::new();
    ///
    /// // Compare version numbers
    /// assert!(version_compare.compare_to("1.2.3", "1.2.3", &CompOp::EQ).unwrap());
    /// assert!(version_compare.compare_to("1.2.3", "1.2.3", &CompOp::LE).unwrap());
    /// assert!(version_compare.compare_to("1.2.3", "1.2.4", &CompOp::LT).unwrap());
    /// assert!(version_compare.compare_to("1", "0.1", &CompOp::GT).unwrap());
    /// assert!(version_compare.compare_to("1", "0.1", &CompOp::GE).unwrap());
    /// ```
    pub fn compare_to(&self, a: &str, b: &str, operator: &CompOp) -> Result<bool, ()> {
        // Create version instances
        let a_ver = Version::from(a);
        let b_ver = Version::from(b);

        // Both version numbers must have been parsed
        if a_ver.is_none() || b_ver.is_none() {
            return Err(());
        }

        // Compare and return the result
        Ok(a_ver.unwrap().compare_to(&b_ver.unwrap(), &operator))
    }
}

#[cfg(test)]
mod tests {
    use comp_op::CompOp;
    use test::test_version::TEST_VERSIONS;
    use test::test_version_set::TEST_VERSION_SETS;
    use super::VersionCompare;

    #[test]
    fn compare() {
        // Create a new version compare instance
        let version_compare = VersionCompare::new();

        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            assert_eq!(
                version_compare.compare(&entry.0, &entry.1),
                Ok(entry.2.clone())
            );
        }
    }

    #[test]
    fn compare_to() {
        // Create a new version compare instance
        let version_compare = VersionCompare::new();

        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Test
            assert!(version_compare.compare_to(&entry.0, &entry.1, &entry.2).unwrap());

            // Make sure the inverse operator is not correct
            assert_eq!(version_compare.compare_to(&entry.0, &entry.1, &entry.2.invert()).unwrap(), false);
        }

        // Assert an exceptional case, compare to not equal
        assert!(version_compare.compare_to("1.2.3", "1.2", &CompOp::NE).unwrap());
    }
}
