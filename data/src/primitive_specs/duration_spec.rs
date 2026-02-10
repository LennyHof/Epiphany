use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// DurationType defines an enumeration that captures the supported duration types.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum DurationType {
    /// Represents a year to month duration.
    YearToMonth,
    /// Represents a day to second duration.
    DayToSecond,
}

impl Display for DurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::YearToMonth => "YearToMonth".to_string(),
                Self::DayToSecond => "DayToSecond".to_string(),
            }
        )
    }
}

/// Resolution of a duration, indicating the smallest unit of time it represents.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum DurationResolution {
    /// Year resolution, the minimum resolution for year-month durations.
    Year,
    /// Month resolution, the maximum resolution for year-month durations.
    Month,
    /// Day resolution, the minimum resolution for day-time durations.
    Day,
    /// Hour resolution
    Hour,
    /// Minute resolution
    Minute,
    /// Second resolution
    Second,
    /// Millisecond resolution
    Millisecond,
    /// Microsecond resolution
    Microsecond,
    /// Nanosecond resolution, the maximum resolution for day-time durations.
    Nanosecond,
}

impl Display for DurationResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Year => "Year",
                Self::Month => "Month",
                Self::Day => "Day",
                Self::Hour => "Hour",
                Self::Minute => "Minute",
                Self::Second => "Second",
                Self::Millisecond => "Millisecond",
                Self::Microsecond => "Microsecond",
                Self::Nanosecond => "Nanosecond",
            }
        )
    }
}

/// A primitive spec for durations.
#[derive(Debug, PartialEq)]
pub struct DurationSpec {
    /// The type of the duration.
    duration_type: Option<DurationType>,
    /// The resolution of the duration.
    resolution: Option<DurationResolution>,
}

impl DurationSpec {
    /// Creates a new duration spec.
    pub(crate) fn new(
        duration_type: Option<DurationType>,
        resolution: Option<DurationResolution>,
    ) -> Self {
        Self {
            duration_type,
            resolution,
        }
    }

    /// Returns the type of the duration.
    pub fn duration_type(&self) -> &Option<DurationType> {
        &self.duration_type
    }

    /// Returns the resolution of the duration.
    pub fn resolution(&self) -> &Option<DurationResolution> {
        &self.resolution
    }

    /// Checks if the duration types of two `DurationSpec`s are compatible.
    fn compatible_duration_type(&self, other: &Self) -> bool {
        match (self.duration_type, other.duration_type) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }

    /// Checks if the resolution types of two `DurationSpec`s are compatible.
    ///
    /// Compatibility rules:
    /// - If both have a resolution, the current spec's resolution must be greater than or equal to the required spec's resolution.
    /// - If the current spec has no resolution but the required spec does, it is not compatible.
    /// - If the required spec has no resolution, it is compatible.
    fn compatible_resolution(&self, other: &Self) -> bool {
        match (self.resolution, other.resolution) {
            (Some(s), Some(r)) => s >= r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl SpecCompatibility for DurationSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        self.compatible_duration_type(required) && self.compatible_resolution(required)
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
            "Duration {{ type: {}, resolution: {} }}",
            self.duration_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.resolution
                .map(|r| r.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
