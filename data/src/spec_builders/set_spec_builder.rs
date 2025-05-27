use std::sync::Arc;

use crate::{
    accessors::collections::set::Set,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::set_spec::SetSpec,
};

/// Builder for set data specifications.
///
/// # Examples
///
/// Create a data specification for a set of integers:
/// ```rust
/// use data::spec_builders::{set_spec_builder::SetSpecBuilder, integer_spec_builder::IntegerSpecBuilder};
/// use data::primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage};
///
/// let set_of_integers_data_spec = SetSpecBuilder::new()
///     .set_element_spec(IntegerSpecBuilder::new()
///         .set_encoding(IntegerEncoding::Signed)
///         .set_storage(IntegerStorage::B64)
///        .build())
///    .build();
/// ```
pub struct SetSpecBuilder {
    element_spec: Option<Arc<DataSpec>>,
}

impl SetSpecBuilder {
    /// Returns an initialized SetSpecBuilder.
    pub fn new() -> SetSpecBuilder {
        SetSpecBuilder { element_spec: None }
    }

    /// Sets the set's element specification.
    /// <p>
    /// Not setting an element specification will result in a set spec with no element specification and thus
    /// can only be used for comparison, not access.
    /// </p>
    pub fn set_element_spec(&mut self, element_spec: Arc<DataSpec>) -> &mut SetSpecBuilder {
        self.element_spec = Some(element_spec);
        self
    }

    /// Builds and returns an initialized set specification.
    pub fn build(&self) -> Arc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<SetSpec, Set>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.element_spec.is_some() {
            let primitive_spec = Arc::new(SetSpec::new(&self.element_spec));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
            specification_level = self.element_spec.as_ref().unwrap().specification_level();
        }
        Arc::new(DataSpec::new_primitive(
            Primitive::Set(primitive_def),
            specification_level,
        ))
    }
}
