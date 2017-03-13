/// Struct containing a version number with some meta data.
/// Such a set can be used for testing.
///
/// # Arguments
///
/// - `0`: The version string.
/// - `1`: Number of version parts.
pub struct TestVersion(
    pub &'static str,
    pub usize
);

/// List of version numbers with metadata for dynamic tests
pub const TEST_VERSIONS: &'static [TestVersion] = &[
    TestVersion("1", 1),
    TestVersion("1.2", 2),
    TestVersion("1.2.3.4", 4),
    TestVersion("1.2.3.4.5.6.7.8", 8),
    TestVersion("0", 1),
    TestVersion("0.0.0", 3),
    TestVersion("1.0.0", 3),
    TestVersion("0.0.1", 3),
    TestVersion("", 0)
];
