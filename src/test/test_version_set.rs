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
    TestVersionSet("1", "1", CompOp::EQ),
    TestVersionSet("1.0.0.0", "1", CompOp::EQ),
    TestVersionSet("1", "1.0.0.0", CompOp::EQ),
    TestVersionSet("0", "0", CompOp::EQ),
    TestVersionSet("0.0.0", "0", CompOp::EQ),
    TestVersionSet("0", "0.0.0", CompOp::EQ),
    TestVersionSet("", "", CompOp::EQ),
    TestVersionSet("", "0.0", CompOp::EQ),
    TestVersionSet("0.0", "", CompOp::EQ),
    TestVersionSet("", "0.1", CompOp::LT),
    TestVersionSet("0.1", "", CompOp::GT),
    TestVersionSet("1.2.3", "1.2.3", CompOp::EQ),
    TestVersionSet("1.2.3", "1.2.4", CompOp::LT),
    TestVersionSet("1.0.0.1", "1.0.0.0", CompOp::GT),
    TestVersionSet("1.0.0.0", "1.0.0.1", CompOp::LT),
    TestVersionSet("1.2.3.4", "1.2", CompOp::GT),
    TestVersionSet("1.2", "1.2.3.4", CompOp::LT),
    TestVersionSet("1.2.3.4", "2", CompOp::LT),
    TestVersionSet("2", "1.2.3.4", CompOp::GT),
    TestVersionSet("123", "123", CompOp::EQ),
    TestVersionSet("123", "1.2.3", CompOp::GT),
    TestVersionSet("1.2.3", "123", CompOp::LT)
];
