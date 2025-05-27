use std::fmt::Display;

use crate::primitive_def::PrimitiveSpec;

/// DateTimeStorage defines an enumeration that captures the supported storage
/// characteristics for DateTime values.
pub enum DateTimeStorage {
    /// DateTime storage (8 bytes).
    DateTime,
    /// DateTimeOffset storage (16 bytes).
    DateTimeOffset,
}

impl Display for DateTimeStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::DateTime => "DateTime".to_string(),
                Self::DateTimeOffset => "DateTimeOffset".to_string(),
            }
        )
    }
}

/// A primitive spec for date-times.
pub struct DateTimeSpec {
    storage: DateTimeStorage,
}

impl DateTimeSpec {
    /// Returns an initialized float spec.
    pub fn new(storage: DateTimeStorage) -> DateTimeSpec {
        DateTimeSpec { storage: (storage) }
    }

    /// Returns the float's storage.
    pub fn storage(&self) -> &DateTimeStorage {
        &self.storage
    }
}

impl PrimitiveSpec for DateTimeSpec {}