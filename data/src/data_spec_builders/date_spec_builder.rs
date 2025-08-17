use crate::{
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::date_spec::DateSpec,
};
use std::rc::Rc;

/// Builder for date data specifications.
///
/// # Examples
///
/// Create a date data specification:
/// ```rust
/// use data::data_spec_builders::date_spec_builder::DateSpecBuilder;
///
/// let date_data_spec = DateSpecBuilder::new()
///   .build();
/// ```
pub struct DateSpecBuilder {}

impl DateSpecBuilder {
    /// Returns an initialized DateSpecBuilder.
    pub fn new() -> DateSpecBuilder {
        DateSpecBuilder {}
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let primitive_spec = Rc::new(DateSpec::new());
        let primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
        Rc::new(DataSpec::new_primitive(
            Primitive::Date(primitive_def),
            DataSpecLevel::Access,
        ))
    }
}
