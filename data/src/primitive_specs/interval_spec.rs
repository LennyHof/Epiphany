use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// IntervalType defines an enumeration that captures the supported interval types.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntervalType {
    /// Represents a year-month interval.
    YearMonth,
    /// Represents a day-time interval.
    DayTime,
}

impl Display for IntervalType {
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

/// A primitive spec for intervals.
#[derive(Debug, PartialEq)]
pub struct IntervalSpec {
    /// The type of the interval.
    interval_type: Option<IntervalType>,
}

impl IntervalSpec {
    /// Creates a new interval spec.
    pub(crate) fn new(interval_type: Option<IntervalType>) -> Self {
        Self { interval_type }
    }

    /// Returns the type of the interval.
    pub fn interval_type(&self) -> &Option<IntervalType> {
        &self.interval_type
    }
}

impl SpecCompatibility for IntervalSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        match (self.interval_type, _required.interval_type) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for IntervalSpec {
    fn is_ordered(&self) -> bool {
        true // Intervals are ordered.
    }
}

impl PrimitiveSpec for IntervalSpec {}

impl std::fmt::Display for IntervalSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Interval {{ type: {} }}",
            self.interval_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
