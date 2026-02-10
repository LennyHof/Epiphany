use crate::{
    accessors::{
        collections::{list::ListError, map::MapError, set::SetError},
        float::FloatError,
        integer::IntegerError,
        sequence::SequenceError,
        temporal::{
            date::DateError, time::TimeError, year_to_month_duration::YearToMonthDurationError,
        },
        tuple::TupleError,
    },
    provider_error::ProviderError,
    spec_compatibility::SpecError,
};

/// A trait for types that can be set equal to another instance of the same type.
pub trait SetEqualTo {
    /// Sets the value of the current instance to the specified value.
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError>;
}
/// An error that can occur when setting a value equal to another value.
#[derive(Debug, PartialEq)]
pub enum SetEqualToError {
    /// An error indicating that the specifications are not compatible.
    SpecError(SpecError),
    /// An integer error.
    IntegerError(IntegerError),
    /// A float error.
    FloatError(FloatError),
    /// A list error.
    ListError(ListError),
    /// A map error.
    MapError(MapError),
    /// A set error.
    SetError(SetError),
    /// A provider error.
    ProviderError(ProviderError),
    /// A sequence error.
    SequenceError(SequenceError),
    /// A tuple error.
    TupleError(TupleError),
    /// A date error.
    DateError(DateError),
    /// A time error.
    TimeError(TimeError),
    /// A year-month duration error.
    YearToMonthDurationError(YearToMonthDurationError),
}

impl From<SpecError> for SetEqualToError {
    fn from(error: SpecError) -> Self {
        SetEqualToError::SpecError(error)
    }
}

impl From<ProviderError> for SetEqualToError {
    fn from(error: ProviderError) -> Self {
        SetEqualToError::ProviderError(error)
    }
}
