use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// TimeType defines an enumeration that captures the supported time types.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeType {
    /// Local time type.
    Local,
    /// Zoned time type.
    Zoned,
}

impl std::fmt::Display for TimeType {
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
/// A primitive spec for times.
#[derive(Debug, PartialEq)]
pub struct TimeSpec {
    time_type: Option<TimeType>,
}

impl TimeSpec {
    /// Returns an initialized Time spec.
    pub(crate) fn new(time_type: Option<TimeType>) -> TimeSpec {
        TimeSpec { time_type }
    }

    /// Returns the time's time_type.
    pub fn time_type(&self) -> &Option<TimeType> {
        &self.time_type
    }
}

impl SpecCompatibility for TimeSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        match (self.time_type, required.time_type) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
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
            "Time {{ time_type: {} }}",
            self.time_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
