//! Version part module.
//!
//! A module that provides the `Part` enum, with the specification of all available version
//! parts. Each version string is broken down into these version parts when being parsed to a
//! `Version`.

use std::borrow::Cow;
use std::fmt;

/// Enum of version string parts.
///
/// Each version string is broken down into these version parts when being parsed to a `Version`.
#[derive(Debug, PartialEq)]
pub enum Part<'a> {
    /// Numeric part, most common in version strings.
    ///
    /// Holds the numerical value.
    Number(i32),

    /// A text part.
    ///
    /// These parts usually hold text with an yet unknown definition. Holds the string slice.
    Text(Cow<'a, str>),
}

impl<'a> fmt::Display for Part<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::Number(n) => write!(f, "{}", n),
            Part::Text(t) => write!(f, "{}", t),
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::Part;

    #[test]
    fn display() {
        assert_eq!(format!("{}", Part::Number(123)), "123");
        assert_eq!(format!("{}", Part::Text("123".into())), "123");
    }
}
