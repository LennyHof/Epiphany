use crate::{
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::boolean_spec::BooleanSpec,
};
use std::rc::Rc;

/// Builder for boolean data specifications.
///
/// # Examples
///
/// Create a boolean data specification:
/// ```rust
/// use data::data_spec_builders::boolean_spec_builder::BooleanSpecBuilder;
/// let boolean_data_spec = BooleanSpecBuilder::new().build();   
/// ```
pub struct BooleanSpecBuilder {}

impl BooleanSpecBuilder {
    /// Returns an initialized BooleanSpecBuilder.
    pub fn new() -> BooleanSpecBuilder {
        BooleanSpecBuilder {}
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let primitive_spec = Rc::new(BooleanSpec::new());
        let primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
        Rc::new(DataSpec::new_primitive(
            Primitive::Boolean(primitive_def),
            DataSpecLevel::Access,
        ))
    }
}
impl Default for BooleanSpecBuilder {
    fn default() -> Self {
        BooleanSpecBuilder::new()
    }
}
