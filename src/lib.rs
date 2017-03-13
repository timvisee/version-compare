mod comp_op;
mod version;
mod version_part;
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
    /// ```ignore
    /// let version_compare = VersionCompare::new();
    ///
    /// // Compare version numbers
    /// assert_eq!(version_compare.compare("1.2.3", "1.2.3"), Ok(CompOp::EQ));
    /// assert_eq!(version_compare.compare("1.2.3", "1.2.4"), Ok(CompOp::LT));
    /// assert_eq!(version_compare.compare("1", "0.1"), Ok(CompOp::GT));
    /// ```
    // TODO: Implement tests!
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
    /// ```ignore
    /// let version_compare = VersionCompare::new();
    ///
    /// // Compare version numbers
    /// assert!(version_compare.compare_to("1.2.3", "1.2.3", &CompOp::EQ));
    /// assert!(version_compare.compare_to("1.2.3", "1.2.3", &CompOp::LE));
    /// assert!(version_compare.compare_to("1.2.3", "1.2.4", &CompOp::LT));
    /// assert!(version_compare.compare_to("1", "0.1", &CompOp::GT));
    /// assert!(version_compare.compare_to("1", "0.1", &CompOp::GE));
    /// ```
    // TODO: Implement tests!
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
    use super::VersionCompare;
    use test::test_version::{TestVersion, TEST_VERSIONS};
    use test::test_version_set::{TestVersionSet, TEST_VERSION_SETS};

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
}
