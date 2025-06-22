use std::rc::Rc;

use crate::{
    accessors::sequence::Sequence,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::sequence_spec::SequenceSpec,
};

/// A builder for sequence data specifications.
///
/// /// # Examples
///
/// Create a data specification for a sequence of integers:
/// ```rust
/// use data::data_spec_builders::{sequence_spec_builder::SequenceSpecBuilder, integer_spec_builder::IntegerSpecBuilder};
/// use data::primitive_specs::{integer_spec::{IntegerEncoding, IntegerStorage}};
///
/// let sequence_of_integers_data_spec = SequenceSpecBuilder::new()
///     .set_value_spec(IntegerSpecBuilder::new()
///         .set_encoding(IntegerEncoding::Signed)
///         .set_storage(IntegerStorage::B64)
///        .build())
///    .build();
/// ```
pub struct SequenceSpecBuilder {
    value_spec: Option<Rc<DataSpec>>,
}

impl SequenceSpecBuilder {
    /// Creates a new SequenceSpecBuilder.
    pub fn new() -> Self {
        Self { value_spec: None }
    }

    /// Sets the value specification for the sequence.
    pub fn set_value_spec(&mut self, value_spec: Rc<DataSpec>) -> &mut Self {
        self.value_spec = Some(value_spec);
        self
    }

    /// Builds and returns an initialized sequence specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<SequenceSpec, Sequence>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.value_spec.is_some() {
            let primitive_spec = Rc::new(SequenceSpec::new(&self.value_spec));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
            let value_spec = self.value_spec.as_ref().unwrap();
            specification_level = value_spec.specification_level();
        }

        Rc::new(DataSpec::new_primitive(
            Primitive::Sequence(primitive_def),
            specification_level,
        ))
    }
}

impl Default for SequenceSpecBuilder {
    fn default() -> Self {
        Self::new()
    }
}
