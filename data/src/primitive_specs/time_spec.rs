use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for times.
pub struct TimeSpec {}

impl TimeSpec {
    /// Returns an initialized Time spec.
    pub fn new() -> TimeSpec {
        TimeSpec {}
    }
}

impl PrimitiveSpec for TimeSpec {}
