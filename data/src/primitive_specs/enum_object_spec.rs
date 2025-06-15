use crate::{primitive_def::PrimitiveSpec, spec_compatibility::SpecCompatibility};

/// A primitive spec for enum objects.
#[derive(Debug, PartialEq)]
pub struct EnumObjectSpec {}

impl EnumObjectSpec {
    /// Returns an initialized EnumObject spec.
    pub fn new() -> EnumObjectSpec {
        EnumObjectSpec {}
    }
}

impl PrimitiveSpec for EnumObjectSpec {}

impl SpecCompatibility for EnumObjectSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        // For now, we assume all EnumObject specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for EnumObjectSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnumObject")
    }
}
