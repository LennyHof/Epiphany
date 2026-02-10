use crate::{
    accessors::temporal::{time::Time, zoned_time::ZonedTime},
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::time_spec::{TimeResolution, TimeSpec, TimeType},
};
use std::rc::Rc;
/// Builder for time data specifications.
///
/// # Examples
/// Create a time data specification without a specific type:
/// ```rust
/// use data::data_spec_builders::time_spec_builder::TimeSpecBuilder;
///
/// let time_data_spec = TimeSpecBuilder::new().build();
/// ```
///
/// Create a time data specification with Local type:
/// ```rust
/// use data::data_spec_builders::time_spec_builder::TimeSpecBuilder;
/// use data::primitive_specs::time_spec::TimeType;
///
/// let time_data_spec = TimeSpecBuilder::new()
///    .set_time_type(TimeType::Local)
///    .build();
/// ```
/// Create a time data specification with Local type and nanosecond resolution:
/// ```rust
/// use data::data_spec_builders::time_spec_builder::TimeSpecBuilder;
/// use data::primitive_specs::time_spec::{TimeType, TimeResolution};
///
/// let time_data_spec = TimeSpecBuilder::new()
///    .set_time_type(TimeType::Local)
///    .set_resolution(TimeResolution::Nanosecond)
///    .build();
/// ```
///
/// Create a time data specification with Zoned type:
/// ```rust
/// use data::data_spec_builders::time_spec_builder::TimeSpecBuilder;
/// use data::primitive_specs::time_spec::TimeType;
///
/// let time_data_spec = TimeSpecBuilder::new()
///     .set_time_type(TimeType::Zoned)
///    .build();
/// ```
/// Create a time data specification with Zoned type and 100 microsecond resolution:
/// ```rust
/// use data::data_spec_builders::time_spec_builder::TimeSpecBuilder;
/// use data::primitive_specs::time_spec::{TimeType, TimeResolution};
///
/// let time_data_spec = TimeSpecBuilder::new()
///     .set_time_type(TimeType::Zoned)
///    .set_resolution(TimeResolution::Microsecond100)
///   .build();
/// ```
pub struct TimeSpecBuilder {
    time_type: Option<TimeType>,
    resolution: Option<TimeResolution>,
}

impl TimeSpecBuilder {
    /// Returns an initialized TimeSpecBuilder.
    pub fn new() -> TimeSpecBuilder {
        TimeSpecBuilder {
            time_type: (None),
            resolution: (None),
        }
    }

    /// Sets the time's type.
    pub fn set_time_type(&mut self, time_type: TimeType) -> &mut TimeSpecBuilder {
        self.time_type = Some(time_type);
        self
    }

    /// Sets the time's resolution.
    pub fn set_resolution(&mut self, resolution: TimeResolution) -> &mut TimeSpecBuilder {
        self.resolution = Some(resolution);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let specification_level = if self.time_type.is_some() {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };
        let time_spec = Rc::new(TimeSpec::new(self.time_type, self.resolution));
        match self.time_type {
            Some(TimeType::Local) => {
                let primitive_def: Option<PrimitiveDef<TimeSpec, Time>> =
                    Some(PrimitiveDef::new(time_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::Time(primitive_def),
                    specification_level,
                ))
            }
            Some(TimeType::Zoned) => {
                let primitive_def: Option<PrimitiveDef<TimeSpec, ZonedTime>> =
                    Some(PrimitiveDef::new(time_spec, None));
                Rc::new(DataSpec::new_primitive(
                    Primitive::ZonedTime(primitive_def),
                    specification_level,
                ))
            }
            None => Rc::new(DataSpec::new_primitive_category(
                crate::primitive_category::PrimitiveCategory::Time,
            )),
        }
    }
}
