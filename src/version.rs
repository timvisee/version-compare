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
}