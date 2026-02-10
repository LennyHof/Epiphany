use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// TimeType defines an enumeration that captures the supported time types.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeType {
    ///  time type.
    Local,
    /// Zoned time type.
    Zoned,
}

impl Display for TimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Local => "Local".to_string(),
                Self::Zoned => "Zoned".to_string(),
            }
        )
    }
}

/// TimeResolution defines an enumeration that captures the supported resolutions for time.
/// <p>
/// The resolutions are ordered from the coarsest (Second) to the finest (Nanosecond).
/// </p>
/// <p>
/// When specified, the resolution indicates the finest granularity supported and exceeds that granularity results in an error.
/// </p>
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TimeResolution {
    /// Second resolution, the minimum resolution for time.
    Second,
    /// Millisecond resolution.
    Millisecond,
    /// 100 Microseconds resolution. An implementation can store time with 100 microsecond resolution in a 32-bit integer.
    Microsecond100,
    /// Microsecond resolution.
    Microsecond,
    /// Nanosecond resolution, the maximum resolution for time. An implementation can store time with nanosecond resolution in a 64-bit integer.
    Nanosecond,
}

impl Display for TimeResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Second => "Second".to_string(),
                Self::Millisecond => "Millisecond".to_string(),
                Self::Microsecond100 => "100 Microseconds".to_string(),
                Self::Microsecond => "Microsecond".to_string(),
                Self::Nanosecond => "Nanosecond".to_string(),
            }
        )
    }
}

/// A primitive spec for times.
#[derive(Debug, PartialEq)]
pub struct TimeSpec {
    time_type: Option<TimeType>,
    resolution: Option<TimeResolution>,
}

impl TimeSpec {
    /// Returns an initialized Time spec.
    pub(crate) fn new(time_type: Option<TimeType>, resolution: Option<TimeResolution>) -> TimeSpec {
        TimeSpec {
            time_type,
            resolution,
        }
    }

    /// Returns the time's time_type.
    pub fn time_type(&self) -> &Option<TimeType> {
        &self.time_type
    }

    /// Returns the time's resolution.
    pub fn resolution(&self) -> &Option<TimeResolution> {
        &self.resolution
    }

    /// Checks if the time type is compatible with the required time type.
    ///
    /// Compatibility rules:
    /// - If both have a time type, they must be equal.
    /// - If the current spec has a time type and the required spec does not, it is compatible.
    /// - If neither has a time type, they are compatible.
    /// - If the current spec does not have a time type but the required spec does, it is not compatible.
    fn compatible_time_type(&self, required: &Self) -> bool {
        match (self.time_type, required.time_type) {
            (Some(s), Some(r)) => s == r,
            (Some(_), None) => true,
            (None, None) => true,
            (None, Some(_)) => false,
        }
    }

    /// Checks if the resolution is compatible with the required resolution.
    ///
    /// Compatibility rules:
    /// - If both have a resolution, the current spec's resolution must be greater than or equal to the required spec's resolution.
    /// - If the current spec has no resolution but the required spec does, it is not compatible.
    /// - If the required spec has no resolution, it is compatible.
    fn compatible_resolution(&self, required: &Self) -> bool {
        match (self.resolution, required.resolution) {
            (Some(s_res), Some(r_res)) => s_res >= r_res,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl SpecCompatibility for TimeSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        self.compatible_time_type(required) && self.compatible_resolution(required)
    }
}

impl IsOrdered for TimeSpec {
    fn is_ordered(&self) -> bool {
        true // Times are ordered.
    }
}

impl PrimitiveSpec for TimeSpec {}

impl std::fmt::Display for TimeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Time {{ type: {}, resolution: {} }}",
            self.time_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.resolution
                .map(|r| r.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
