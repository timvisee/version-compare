use crate::Cmp;

/// Struct containing two version numbers, and the comparison operator.
/// Such a set can be used for testing.
///
/// # Arguments
///
/// - `0`: The main version.
/// - `1`: The other version.
/// - `2`: The comparison operator.
pub struct TestVersionSet(pub &'static str, pub &'static str, pub Cmp);

/// List of version sets for dynamic tests
pub const TEST_VERSION_SETS: &'static [TestVersionSet] = &[
    TestVersionSet("1", "1", Cmp::Eq),
    TestVersionSet("1.0.0.0", "1", Cmp::Eq),
    TestVersionSet("1", "1.0.0.0", Cmp::Eq),
    TestVersionSet("0", "0", Cmp::Eq),
    TestVersionSet("0.0.0", "0", Cmp::Eq),
    TestVersionSet("0", "0.0.0", Cmp::Eq),
    TestVersionSet("", "", Cmp::Eq),
    TestVersionSet("", "0.0", Cmp::Eq),
    TestVersionSet("0.0", "", Cmp::Eq),
    TestVersionSet("", "0.1", Cmp::Lt),
    TestVersionSet("0.1", "", Cmp::Gt),
    TestVersionSet("1.2.3", "1.2.3", Cmp::Eq),
    TestVersionSet("1.2.3", "1.2.4", Cmp::Lt),
    TestVersionSet("1.0.0.1", "1.0.0.0", Cmp::Gt),
    TestVersionSet("1.0.0.0", "1.0.0.1", Cmp::Lt),
    TestVersionSet("1.2.3.4", "1.2", Cmp::Gt),
    TestVersionSet("1.2", "1.2.3.4", Cmp::Lt),
    TestVersionSet("1.2.3.4", "2", Cmp::Lt),
    TestVersionSet("2", "1.2.3.4", Cmp::Gt),
    TestVersionSet("123", "123", Cmp::Eq),
    TestVersionSet("123", "1.2.3", Cmp::Gt),
    TestVersionSet("1.2.3", "123", Cmp::Lt),
    TestVersionSet("1.1.2", "1.1.30-dev", Cmp::Lt),
    TestVersionSet("1.2.3", "1.2.3.alpha", Cmp::Gt),
    TestVersionSet("1.2.3", "1.2.3-dev", Cmp::Gt),
    TestVersionSet("1.2.3 RC0", "1.2.3 rc1", Cmp::Lt),
    TestVersionSet("1.2.3 rc2", "1.2.3 RC99", Cmp::Lt),
    TestVersionSet("1.2.3 RC3", "1.2.3 RC1", Cmp::Gt),
    TestVersionSet("1.2.3a", "1.2.3b", Cmp::Lt),
    TestVersionSet("1.2.3b", "1.2.3a", Cmp::Gt),
    TestVersionSet("1.2.3.dev", "1.2.3.alpha", Cmp::Gt),
    TestVersionSet("1.2.3-dev", "1.2.3-alpha", Cmp::Gt),
    TestVersionSet("1.2.3.dev.1", "1.2.3.alpha", Cmp::Gt),
    TestVersionSet("1.2.3-dev-1", "1.2.3-alpha", Cmp::Gt),
    TestVersionSet("version-compare 3.2.0 / build 0932", "3.2.5", Cmp::Lt),
    TestVersionSet("version-compare 3.2.0 / build 0932", "3.1.1", Cmp::Gt),
    TestVersionSet(
        "version-compare 1.4.1 / build 0043",
        "version-compare 1.4.1 / build 0043",
        Cmp::Eq,
    ),
    TestVersionSet(
        "version-compare 1.4.1 / build 0042",
        "version-compare 1.4.1 / build 0043",
        Cmp::Lt,
    ),
    // Issue: https://github.com/timvisee/version-compare/issues/24
    TestVersionSet("7.2p1", "7.1", Cmp::Gt),
    // TODO: inspect these cases
    TestVersionSet("snapshot.1.2.3", "1.2.3.alpha", Cmp::Lt),
    TestVersionSet("snapshot-1.2.3", "1.2.3-alpha", Cmp::Lt),
];

/// List of invalid version sets for dynamic tests
pub const TEST_VERSION_SETS_ERROR: &'static [TestVersionSet] = &[
    TestVersionSet("1.2.3", "1.2.3", Cmp::Lt),
    TestVersionSet("1.2", "1.2.0.0", Cmp::Ne),
    TestVersionSet("1.2.3.dev", "dev", Cmp::Eq),
    TestVersionSet("snapshot", "1", Cmp::Lt),
];
