use comp_op::CompOp;

/// Struct containing two version numbers, and the comparison operator.
/// Such a set can be used for testing.
///
/// # Arguments
///
/// - `0`: The main version.
/// - `1`: The other version.
/// - `2`: The comparison operator.
pub struct TestVersionSet(
    pub &'static str,
    pub &'static str,
    pub CompOp
);

/// List of version sets for dynamic tests
pub const TEST_VERSION_SETS: &'static [TestVersionSet] = &[
    TestVersionSet("1", "1", CompOp::Eq),
    TestVersionSet("1.0.0.0", "1", CompOp::Eq),
    TestVersionSet("1", "1.0.0.0", CompOp::Eq),
    TestVersionSet("0", "0", CompOp::Eq),
    TestVersionSet("0.0.0", "0", CompOp::Eq),
    TestVersionSet("0", "0.0.0", CompOp::Eq),
    TestVersionSet("", "", CompOp::Eq),
    TestVersionSet("", "0.0", CompOp::Eq),
    TestVersionSet("0.0", "", CompOp::Eq),
    TestVersionSet("", "0.1", CompOp::Lt),
    TestVersionSet("0.1", "", CompOp::Gt),
    TestVersionSet("1.2.3", "1.2.3", CompOp::Eq),
    TestVersionSet("1.2.3", "1.2.4", CompOp::Lt),
    TestVersionSet("1.0.0.1", "1.0.0.0", CompOp::Gt),
    TestVersionSet("1.0.0.0", "1.0.0.1", CompOp::Lt),
    TestVersionSet("1.2.3.4", "1.2", CompOp::Gt),
    TestVersionSet("1.2", "1.2.3.4", CompOp::Lt),
    TestVersionSet("1.2.3.4", "2", CompOp::Lt),
    TestVersionSet("2", "1.2.3.4", CompOp::Gt),
    TestVersionSet("123", "123", CompOp::Eq),
    TestVersionSet("123", "1.2.3", CompOp::Gt),
    TestVersionSet("1.2.3", "123", CompOp::Lt),
    TestVersionSet("1.2.3", "1.2.3.alpha", CompOp::Eq),
    TestVersionSet("1.2.3.dev", "1.2.3.alpha", CompOp::Eq),
    TestVersionSet("1.2.3.dev.1", "1.2.3.alpha", CompOp::Gt),
    TestVersionSet("snapshot.1.2.3", "1.2.3.alpha", CompOp::Eq)
];
