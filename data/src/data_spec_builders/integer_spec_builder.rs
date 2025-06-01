use crate::{
    accessors::integer::Integer,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::integer_spec::{IntegerEncoding, IntegerSpec, IntegerStorage},
};
use std::sync::Arc;

/// Builder for integer data specifications.
///
/// # Examples
///
/// Create an integer data specification with signed encoding and base64 storage:
/// ```rust
/// use data::data_spec_builders::integer_spec_builder::IntegerSpecBuilder;
/// use data::primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage};
///
/// let integer_data_spec = IntegerSpecBuilder::new()
///     .set_encoding(IntegerEncoding::Signed)
///     .set_storage(IntegerStorage::B64)
///     .build();
/// ```
pub struct IntegerSpecBuilder {
    encoding: Option<IntegerEncoding>,
    storage: Option<IntegerStorage>,
}

impl IntegerSpecBuilder {
    /// Returns an initialized IntegerSpecBuilder.
    pub fn new() -> IntegerSpecBuilder {
        IntegerSpecBuilder {
            encoding: (None),
            storage: (None),
        }
    }

    /// Sets the integer's encoding.
    pub fn set_encoding(&mut self, encoding: IntegerEncoding) -> &mut IntegerSpecBuilder {
        self.encoding = Some(encoding);
        self
    }

    /// Sets the integer's storage.
    pub fn set_storage(&mut self, storage: IntegerStorage) -> &mut IntegerSpecBuilder {
        self.storage = Some(storage);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Arc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<IntegerSpec, Integer>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.encoding.is_some() || self.storage.is_some() {
            let primitive_spec = Arc::new(IntegerSpec::new(self.encoding, self.storage));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
            if self.encoding.is_some() && self.storage.is_some() {
                specification_level = DataSpecLevel::Access;
            }
        }
        Arc::new(DataSpec::new_primitive(
            Primitive::Integer(primitive_def),
            specification_level,
        ))
    }
}

impl Default for IntegerSpecBuilder {
    fn default() -> Self {
        IntegerSpecBuilder::new()
    }
}
