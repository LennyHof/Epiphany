use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for GUIDs.
pub struct GuidSpec {}

impl GuidSpec {
    /// Returns an initialized GUID spec.
    pub fn new() -> GuidSpec {
        GuidSpec {}
    }
}

impl Default for GuidSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveSpec for GuidSpec {}
