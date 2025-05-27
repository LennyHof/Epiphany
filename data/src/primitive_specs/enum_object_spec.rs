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
