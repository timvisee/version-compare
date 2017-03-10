/// Version struct. A wrapper for a version number string.
pub struct Version<'a> {
    version: &'a str
}

/// Version struct implementation.
impl<'a> Version<'a> {

    /// Constructor.
    pub fn new(version: &'a str) -> Self {
        Version {
            version: version
        }
    }

    /// Get the version string.
    pub fn version(&self) -> &str {
        &self.version
    }
}