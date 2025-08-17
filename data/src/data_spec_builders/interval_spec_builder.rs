use crate::{
    accessors::temporal::day_second_interval::DaySecondInterval,
    accessors::temporal::year_month_interval::YearMonthInterval,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::interval_spec::{IntervalSpec, IntervalType},
};
use std::rc::Rc;

/// Builder for interval data specifications.
///
/// # Examples
///
/// Create an interval data specification without a specific type:
/// ```rust
/// use data::data_spec_builders::interval_spec_builder::IntervalSpecBuilder;
///
/// let interval_spec = IntervalSpecBuilder::new().build();
/// ```
/// Create an interval data specification with YearMonth type:
/// ```rust
/// use data::data_spec_builders::interval_spec_builder::IntervalSpecBuilder;
/// use data::primitive_specs::interval_spec::IntervalType;
///
/// let interval_spec = IntervalSpecBuilder::new()
///     .set_interval_type(IntervalType::YearMonth)
///     .build();
/// ```
/// Create an interval data specification with DayTime type:
/// ```rust
/// use data::data_spec_builders::interval_spec_builder::IntervalSpecBuilder;
/// use data::primitive_specs::interval_spec::IntervalType;
///
/// let interval_spec = IntervalSpecBuilder::new()
///     .set_interval_type(IntervalType::DayTime)
///     .build();
/// ```
pub struct IntervalSpecBuilder {
    interval_type: Option<IntervalType>,
}
impl IntervalSpecBuilder {
    /// Returns an initialized IntervalSpecBuilder.
    pub fn new() -> IntervalSpecBuilder {
        IntervalSpecBuilder {
            interval_type: None,
        }
    }

    /// Sets the interval's type.
    pub fn set_interval_type(&mut self, interval_type: IntervalType) -> &mut IntervalSpecBuilder {
        self.interval_type = Some(interval_type);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let specification_level = if self.interval_type.is_some() {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };
        let primitive_spec = Rc::new(IntervalSpec::new(self.interval_type));
        match self.interval_type {
            Some(IntervalType::YearMonth) => {
                let primitive_def: Option<PrimitiveDef<IntervalSpec, YearMonthInterval>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::YearMonthInterval(primitive_def),
                    specification_level,
                ))
            }
            Some(IntervalType::DayTime) => {
                let primitive_def: Option<PrimitiveDef<IntervalSpec, DaySecondInterval>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::DaySecondInterval(primitive_def),
                    specification_level,
                ))
            }
            None => Rc::new(DataSpec::new_primitive_category(
                crate::primitive_category::PrimitiveCategory::Interval,
            )),
        }
    }
}
