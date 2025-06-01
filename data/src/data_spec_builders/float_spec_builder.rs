use crate::{
    accessors::float::Float,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::float_spec::{FloatSpec, FloatStorage},
};
use std::sync::Arc;

/// Builder for float data specifications.
///
/// # Examples
///
/// Create a float data specification with base64 storage:
/// ```rust
/// use data::data_spec_builders::float_spec_builder::FloatSpecBuilder;
/// use data::primitive_specs::float_spec::FloatStorage;
/// let float_data_spec = FloatSpecBuilder::new()
///    .set_storage(FloatStorage::B64)
///    .build();
/// ```
pub struct FloatSpecBuilder {
    storage: Option<FloatStorage>,
}

impl FloatSpecBuilder {
    /// Returns an initialized FloatSpecBuilder.
    pub fn new() -> FloatSpecBuilder {
        FloatSpecBuilder { storage: (None) }
    }

    /// Sets the float's storage.
    pub fn set_storage(&mut self, storage: FloatStorage) -> &mut FloatSpecBuilder {
        self.storage = Some(storage);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Arc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<FloatSpec, Float>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.storage.is_some() {
            let primitive_spec = Arc::new(FloatSpec::new(self.storage));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
            specification_level = DataSpecLevel::Access;
        }
        Arc::new(DataSpec::new_primitive(
            Primitive::Float(primitive_def),
            specification_level,
        ))
    }
}
impl Default for FloatSpecBuilder {
    fn default() -> Self {
        FloatSpecBuilder::new()
    }
}
