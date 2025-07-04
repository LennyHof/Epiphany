use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// FloatStorage defines an enumeration that captures the supported
/// characteristics for IEEE floats.
/// <p>
/// These values correspond to the size in bytes for each type; do not change
/// as this assumption is used in other places.
/// </p>
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FloatStorage {
    /// Four bytes per float.
    B32 = 4,
    /// Eight bytes per float.
    B64 = 8,
}

impl Display for FloatStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::B32 => "B32".to_string(),
                Self::B64 => "B64".to_string(),
            }
        )
    }
}

/// A primitive spec for floats.
#[derive(Debug, PartialEq)]
pub struct FloatSpec {
    storage: Option<FloatStorage>,
}

impl FloatSpec {
    pub(crate) fn new(storage: Option<FloatStorage>) -> FloatSpec {
        FloatSpec { storage: (storage) }
    }

    /// Returns the float's IEEE storage.
    pub fn storage(&self) -> &Option<FloatStorage> {
        &self.storage
    }
}

impl SpecCompatibility for FloatSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        match (self.storage, required.storage) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for FloatSpec {
    fn is_ordered(&self) -> bool {
        // Floats are ordered.
        true
    }
}

impl PrimitiveSpec for FloatSpec {}

impl Display for FloatSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Float {{ storage: {} }}",
            self.storage
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
