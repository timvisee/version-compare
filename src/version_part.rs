/// Part of a version string.
#[derive(Debug, PartialEq)]
pub enum VersionPart<'a> {

    /// Numeric part.
    /// Holds the numerical value.
    Number(i32),

    /// A text part.
    /// These parts usually have an unknown definition.
    /// Holds the string slice.
    Text(&'a str)
}
