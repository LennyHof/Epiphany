use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// DurationType defines an enumeration that captures the supported duration types.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DurationType {
    /// Represents a year-month duration.
    YearMonth,
    /// Represents a day-time duration.
    DayTime,
}

impl Display for DurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::YearMonth => "YearMonth".to_string(),
                Self::DayTime => "DayTime".to_string(),
            }
        )
    }
}

/// A primitive spec for durations.
#[derive(Debug, PartialEq)]
pub struct DurationSpec {
    /// The type of the duration.
    duration_type: Option<DurationType>,
}

impl DurationSpec {
    /// Creates a new duration spec.
    pub(crate) fn new(duration_type: Option<DurationType>) -> Self {
        Self { duration_type }
    }

    /// Returns the type of the duration.
    pub fn duration_type(&self) -> &Option<DurationType> {
        &self.duration_type
    }
}

impl SpecCompatibility for DurationSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        match (self.duration_type, _required.duration_type) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for DurationSpec {
    fn is_ordered(&self) -> bool {
        true // Durations are ordered.
    }
}

impl PrimitiveSpec for DurationSpec {}

impl std::fmt::Display for DurationSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Duration {{ type: {} }}",
            self.duration_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
