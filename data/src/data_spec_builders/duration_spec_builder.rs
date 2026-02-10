use crate::{
    accessors::temporal::day_to_second_duration::DayToSecondDuration,
    accessors::temporal::year_to_month_duration::YearToMonthDuration,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::duration_spec::{DurationResolution, DurationSpec, DurationType},
};
use std::{fmt::Display, rc::Rc};

/// Builder for duration data specifications.
///
/// # Examples
///
/// Create an duration data specification without a specific type:
/// ```rust
/// use data::data_spec_builders::duration_spec_builder::DurationSpecBuilder;
///
/// let duration_spec = DurationSpecBuilder::new().build();
/// ```
/// Create an duration data specification with YearToMonth type:
/// ```rust
/// use data::data_spec_builders::duration_spec_builder::DurationSpecBuilder;
/// use data::primitive_specs::duration_spec::DurationType;
///
/// let duration_spec = DurationSpecBuilder::new()
///     .set_type(DurationType::YearToMonth)
///     .build();
/// ```
/// Create an duration data specification with DayToSecond type:
/// ```rust
/// use data::data_spec_builders::duration_spec_builder::DurationSpecBuilder;
/// use data::primitive_specs::duration_spec::DurationType;
///
/// let duration_spec = DurationSpecBuilder::new()
///     .set_type(DurationType::DayToSecond)
///     .build();
/// ```
/// Create an duration data specification with a DayToSecond type and a millisecond resolution:
/// ```rust
/// use data::data_spec_builders::duration_spec_builder::DurationSpecBuilder;
/// use data::primitive_specs::duration_spec::{DurationType, DurationResolution};
///
/// let duration_spec = DurationSpecBuilder::new()
///     .set_type(DurationType::DayToSecond)
///     .set_resolution(DurationResolution::Millisecond)
///     .build();
/// ```
pub struct DurationSpecBuilder {
    duration_type: Option<DurationType>,
    resolution: Option<DurationResolution>,
}
impl DurationSpecBuilder {
    /// Returns an initialized DurationSpecBuilder.
    pub fn new() -> DurationSpecBuilder {
        DurationSpecBuilder {
            duration_type: None,
            resolution: None,
        }
    }

    /// Sets the duration's type.
    pub fn set_type(&mut self, duration_type: DurationType) -> &mut DurationSpecBuilder {
        self.duration_type = Some(duration_type);
        self
    }

    /// Sets the duration's resolution.
    pub fn set_resolution(&mut self, resolution: DurationResolution) -> &mut DurationSpecBuilder {
        self.resolution = Some(resolution);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Result<Rc<DataSpec>, DurationSpecBuildError> {
        let specification_level = if self.duration_type.is_some() {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };
        let primitive_spec = Rc::new(DurationSpec::new(self.duration_type, self.resolution));
        match self.duration_type {
            Some(DurationType::YearToMonth) => {
                if let Some(resolution) = self.resolution {
                    match resolution {
                        DurationResolution::Year | DurationResolution::Month => {}
                        _ => {
                            return Err(DurationSpecBuildError::IncompatibleResolution(
                                DurationType::YearToMonth,
                                resolution,
                            ));
                        }
                    }
                }
                let primitive_def: Option<PrimitiveDef<DurationSpec, YearToMonthDuration>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Ok(Rc::new(DataSpec::new_primitive(
                    Primitive::YearToMonthDuration(primitive_def),
                    specification_level,
                )))
            }
            Some(DurationType::DayToSecond) => {
                if let Some(resolution) = self.resolution {
                    match resolution {
                        DurationResolution::Day
                        | DurationResolution::Hour
                        | DurationResolution::Minute
                        | DurationResolution::Second
                        | DurationResolution::Millisecond
                        | DurationResolution::Microsecond
                        | DurationResolution::Nanosecond => {}
                        _ => {
                            return Err(DurationSpecBuildError::IncompatibleResolution(
                                DurationType::DayToSecond,
                                resolution,
                            ));
                        }
                    }
                }
                let primitive_def: Option<PrimitiveDef<DurationSpec, DayToSecondDuration>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Ok(Rc::new(DataSpec::new_primitive(
                    Primitive::DayToSecondDuration(primitive_def),
                    specification_level,
                )))
            }
            None => {
                if self.resolution.is_some() {
                    return Err(DurationSpecBuildError::ResulutionWithoutDurationType);
                }
                Ok(Rc::new(DataSpec::new_primitive_category(
                    crate::primitive_category::PrimitiveCategory::Duration,
                )))
            }
        }
    }
}

/// Errors that can occur when building `DurationSpec`s.
#[derive(Debug, PartialEq)]
pub enum DurationSpecBuildError {
    /// Indicates that the specified resolution is not compatible with the given duration type.
    IncompatibleResolution(DurationType, DurationResolution),

    /// Indicates that a resolution was specified without a duration type.
    ResulutionWithoutDurationType,
}

impl Display for DurationSpecBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationSpecBuildError::IncompatibleResolution(duration_type, resolution) => {
                write!(
                    f,
                    "Incompatible resolution ({}) for duration type ({}).",
                    resolution, duration_type
                )
            }
            DurationSpecBuildError::ResulutionWithoutDurationType => {
                write!(f, "Cannot set resolution without setting duration type.")
            }
        }
    }
}

impl std::error::Error for DurationSpecBuildError {}
