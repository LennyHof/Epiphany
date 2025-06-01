use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for enum objects.
pub struct EnumObjectSpec {}

impl EnumObjectSpec {
    /// Returns an initialized EnumObject spec.
    pub fn new() -> EnumObjectSpec {
        EnumObjectSpec {}
    }
}

impl PrimitiveSpec for EnumObjectSpec {}

impl EnumObjectSpec {
    /// Returns if this EnumObject spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all EnumObject specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}
