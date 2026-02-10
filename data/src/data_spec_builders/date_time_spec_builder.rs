use crate::{
    accessors::temporal::{date_time::DateTime, zoned_date_time::ZonedDateTime},
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::date_time_spec::{DateTimeSpec, DateTimeType},
};
use std::rc::Rc;
/// Builder for date-time data specifications.
///
/// # Examples
///
/// Create a date-time data specification without a specific type:
/// ```rust
/// use data::data_spec_builders::date_time_spec_builder::DateTimeSpecBuilder;
/// let date_time_spec = DateTimeSpecBuilder::new().build();
/// ```
/// Create a date-time data specification with Local type:
/// ```rust
/// use data::data_spec_builders::date_time_spec_builder::DateTimeSpecBuilder;
/// use data::primitive_specs::date_time_spec::DateTimeType;
/// let date_time_data_spec = DateTimeSpecBuilder::new()
///    .set_date_time_type(DateTimeType::Local)
///   .build();
/// ```
/// Create a date-time data specification with Zoned type:
/// ```rust
/// use data::data_spec_builders::date_time_spec_builder::DateTimeSpecBuilder;
/// use data::primitive_specs::date_time_spec::DateTimeType;
/// let date_time_spec = DateTimeSpecBuilder::new()
///     .set_date_time_type(DateTimeType::Zoned)
/// .build();
/// ```
pub struct DateTimeSpecBuilder {
    date_time_type: Option<DateTimeType>,
}

impl DateTimeSpecBuilder {
    /// Returns an initialized DateTimeSpecBuilder.
    pub fn new() -> DateTimeSpecBuilder {
        DateTimeSpecBuilder {
            date_time_type: (None),
        }
    }

    /// Sets the date-time's type.
    pub fn set_date_time_type(&mut self, date_time_type: DateTimeType) -> &mut DateTimeSpecBuilder {
        self.date_time_type = Some(date_time_type);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let specification_level = if self.date_time_type.is_some() {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };
        let primitive_spec = Rc::new(DateTimeSpec::new(self.date_time_type));
        match self.date_time_type {
            Some(DateTimeType::Local) => {
                let primitive_def: Option<PrimitiveDef<DateTimeSpec, DateTime>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::DateTime(primitive_def),
                    specification_level,
                ))
            }
            Some(DateTimeType::Zoned) => {
                let primitive_def: Option<PrimitiveDef<DateTimeSpec, ZonedDateTime>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::ZonedDateTime(primitive_def),
                    specification_level,
                ))
            }
            None => Rc::new(DataSpec::new_primitive_category(
                crate::primitive_category::PrimitiveCategory::DateTime,
            )),
        }
    }
}
