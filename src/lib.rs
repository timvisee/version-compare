mod comp_op;
mod version;
mod version_part;

use comp_op::CompOp;
use version::Version;

/// Version compare structure.
pub struct VersionCompare { }

/// Version compare implementation.
impl VersionCompare {

    /// Create a new version compare instance.
    pub fn new() -> Self {
        VersionCompare { }
    }
}
