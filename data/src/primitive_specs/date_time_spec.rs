use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// DateTimeType defines an enumeration that captures the supported date time types.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DateTimeType {
    /// Local date time type.
    Local,
    /// Zoned date time type.
    Zoned,
}

impl Display for DateTimeType {
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

/// A primitive spec for date-times.
#[derive(Debug, PartialEq)]
pub struct DateTimeSpec {
    date_time_type: Option<DateTimeType>,
}

impl DateTimeSpec {
    /// Returns an initialized date-time spec.
    pub(crate) fn new(date_time_type: Option<DateTimeType>) -> DateTimeSpec {
        DateTimeSpec { date_time_type }
    }

    /// Returns the date-time's date_time_type.
    pub fn date_time_type(&self) -> &Option<DateTimeType> {
        &self.date_time_type
    }
}

impl SpecCompatibility for DateTimeSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        match (self.date_time_type, required.date_time_type) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for DateTimeSpec {
    fn is_ordered(&self) -> bool {
        // DateTime values are ordered.
        true
    }
}

impl PrimitiveSpec for DateTimeSpec {}

impl Display for DateTimeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DateTime {{ type: {} }}",
            self.date_time_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
