use crate::{
    accessors::temporal::day_second_duration::DaySecondDuration,
    accessors::temporal::year_month_duration::YearMonthDuration,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::duration_spec::{DurationSpec, DurationType},
};
use std::rc::Rc;

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
/// Create an duration data specification with YearMonth type:
/// ```rust
/// use data::data_spec_builders::duration_spec_builder::DurationSpecBuilder;
/// use data::primitive_specs::duration_spec::DurationType;
///
/// let duration_spec = DurationSpecBuilder::new()
///     .set_duration_type(DurationType::YearMonth)
///     .build();
/// ```
/// Create an duration data specification with DayTime type:
/// ```rust
/// use data::data_spec_builders::duration_spec_builder::DurationSpecBuilder;
/// use data::primitive_specs::duration_spec::DurationType;
///
/// let duration_spec = DurationSpecBuilder::new()
///     .set_duration_type(DurationType::DayTime)
///     .build();
/// ```
pub struct DurationSpecBuilder {
    duration_type: Option<DurationType>,
}
impl DurationSpecBuilder {
    /// Returns an initialized DurationSpecBuilder.
    pub fn new() -> DurationSpecBuilder {
        DurationSpecBuilder {
            duration_type: None,
        }
    }

    /// Sets the duration's type.
    pub fn set_duration_type(&mut self, duration_type: DurationType) -> &mut DurationSpecBuilder {
        self.duration_type = Some(duration_type);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let specification_level = if self.duration_type.is_some() {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };
        let primitive_spec = Rc::new(DurationSpec::new(self.duration_type));
        match self.duration_type {
            Some(DurationType::YearMonth) => {
                let primitive_def: Option<PrimitiveDef<DurationSpec, YearMonthDuration>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::YearMonthDuration(primitive_def),
                    specification_level,
                ))
            }
            Some(DurationType::DayTime) => {
                let primitive_def: Option<PrimitiveDef<DurationSpec, DaySecondDuration>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::DaySecondDuration(primitive_def),
                    specification_level,
                ))
            }
            None => Rc::new(DataSpec::new_primitive_category(
                crate::primitive_category::PrimitiveCategory::Duration,
            )),
        }
    }
}
