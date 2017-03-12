use std::iter::Peekable;
use std::slice::Iter;

use comp_op::CompOp;
use version_part::VersionPart;

/// Version struct. A wrapper for a version number string.
pub struct Version<'a> {
    version: &'a str,
    parts: Vec<VersionPart<'a>>
}

/// Version struct implementation.
impl<'a> Version<'a> {

    /// Create a `Version` instance from a version string.
    ///
    /// The version string should be passed to the `version` parameter.
    pub fn from(version: &'a str) -> Option<Self> {
        // Split the version string
        let parts = Self::split_version_str(version);

        // Return nothing if the parts are none
        if parts.is_none() {
            return None;
        }

        // Create and return the object
        Some(Version {
            version: version,
            parts: parts.unwrap()
        })
    }

    /// Split the given version string, in it's version parts.
    /// TODO: Move this method to some sort of helper class, maybe as part of `VersionPart`.
    fn split_version_str(version: &'a str) -> Option<Vec<VersionPart>> {
        // Split the version string, and create a vector to put the parts in
        let split = version.split('.');
        let mut parts = Vec::new();

        // Loop over the parts, and parse them
        for part in split {
            // Try to parse the value as an number
            match part.parse::<i32>() {
                Ok(number) => parts.push(VersionPart::Number(number)),
                Err(_) => parts.push(VersionPart::Text(part))
            }
        }

        // Return the list of parts
        Some(parts)
    }

    /// Get the original version string.
    pub fn as_str(&self) -> &str {
        &self.version
    }

    /// Get the number of parts in this version string.
    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    /// Compare this version to the given `other` version.
    ///
    /// This method returns one of the following comparison operators:
    /// - LT
    /// - EQ
    /// - GT
    ///
    /// # Examples:
    ///
    /// ```ignore
    /// assert_eq!(Version::from("1.2").compare(Version::from("1.3.2")), CompOp::LT);
    /// assert_eq!(Version::from("1.9").compare(Version::from("1.9")), CompOp::EQ);
    /// assert_eq!(Version::from("0.3.0.0").compare(Version::from("0.3")), CompOp::EQ);
    /// assert_eq!(Version::from("2").compare(Version::from("1.7.3")), CompOp::GT);
    /// ```
    pub fn compare(&self, other: &Version) -> CompOp {
        // Compare the versions with their peekable iterators
        Self::compare_iter(
            self.parts.iter().peekable(),
            other.parts.iter().peekable()
        )
    }

    /// Compare two version numbers based on the iterators of their version parts.
    ///
    /// This method returns one of the following comparison operators:
    /// - LT
    /// - EQ
    /// - GT
    fn compare_iter(mut iter: Peekable<Iter<VersionPart<'a>>>, mut other_iter: Peekable<Iter<VersionPart<'a>>>) -> CompOp {
        // Iterate through the parts of this version
        let mut other_part: Option<&VersionPart> = None;

        // Iterate over the iterator, without consuming it
        loop {
            match iter.next() {
                Some(part) => {
                    // Skip this part if it's non-numeric
                    match part {
                        &VersionPart::Number(_) => {},
                        _ => continue
                    }

                    // Get the next numerical part for the other version
                    loop {
                        match other_iter.next() {
                            Some(val) =>
                                match val {
                                    &VersionPart::Number(_) => {
                                        other_part = Some(val);
                                        break;
                                    },
                                    _ => {}
                                },
                            None => {}
                        }
                    }

                    // If there are no parts left in the other version, try to determine the result
                    if other_part.is_none() {
                        // In the main version: if the current part is zero, continue to the next one
                        match part {
                            &VersionPart::Number(num) =>
                                if num == 0 {
                                    continue;
                                },
                            _ => {}
                        }

                        // The main version is greater
                        return CompOp::GT;
                    }

                    // Match both part as numbers to destruct their numerical values
                    match part {
                        &VersionPart::Number(num) =>
                            match other_part.unwrap() {
                                &VersionPart::Number(other_num) => {
                                    // Compare the numbers
                                    match num {
                                        n if n < other_num => return CompOp::LT,
                                        n if n > other_num => return CompOp::GT,
                                        n if n == other_num => continue,

                                        // This part can't be reached
                                        _ => panic!()
                                    }
                                },

                                // This part can't be reached
                                _ => panic!()
                            }
                        ,

                        // This part can't be reached
                        _ => panic!()
                    }
                },
                None => break
            }
        }

        // Check whether we should iterate over the other iterator, if it has any items left
        match other_iter.peek() {
            // Compare based on the other iterator
            Some(_) => Self::compare_iter(other_iter, iter).as_flipped(),

            // Nothing more to iterate over, the versions should be equal
            None => CompOp::EQ
        }
    }
}
