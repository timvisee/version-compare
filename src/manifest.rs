//! Module for the version manifest.
//!
//! A version manifest can be used to configure and specify how versions are parsed and compared.
//! For example, you can configure the maximum depth of a version number, and set whether text
//! parts are ignored in a version string.

/// Version manifest (configuration).
///
/// A manifest (configuration) that is used respectively when parsing and comparing version strings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Manifest {
    /// The maximum depth of a version number.
    ///
    /// This specifies the maximum number of parts. There is no limit if `None` is set.
    pub max_depth: Option<usize>,

    /// Whether to ignore text parts in version strings.
    pub ignore_text: bool,
}

/// Version manifest implementation.
impl Manifest {
    /// Check whether there's a maximum configured depth.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Manifest;
    ///
    /// let mut manifest = Manifest::default();
    ///
    /// assert!(!manifest.has_max_depth());
    ///
    /// manifest.max_depth = Some(3);
    /// assert!(manifest.has_max_depth());
    /// ```
    pub fn has_max_depth(&self) -> bool {
        self.max_depth.is_some() && self.max_depth.unwrap() > 0
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::Manifest;

    #[test]
    fn has_max_depth() {
        let mut manifest = Manifest::default();

        manifest.max_depth = Some(1);
        assert!(manifest.has_max_depth());

        manifest.max_depth = Some(3);
        assert!(manifest.has_max_depth());

        manifest.max_depth = None;
        assert!(!manifest.has_max_depth());
    }
}
