use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for dates.
pub struct DateSpec {}

impl DateSpec {
    /// Returns an initialized Date spec.
    pub fn new() -> DateSpec {
        DateSpec {}
    }
}

impl PrimitiveSpec for DateSpec {}
