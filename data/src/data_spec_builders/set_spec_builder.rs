use std::rc::Rc;

use crate::{
    accessors::collections::set::Set,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::{IsOrdered, PrimitiveDef},
    primitive_specs::set_spec::{SetSpec, SetStorage},
};

/// Builder for set data specifications.
///
/// # Examples
///
/// Create a data specification for an unordered set of integers:
/// ```rust
/// use data::data_spec_builders::{set_spec_builder::SetSpecBuilder, integer_spec_builder::IntegerSpecBuilder};
/// use data::primitive_specs::{integer_spec::{IntegerEncoding, IntegerStorage}, set_spec::SetStorage};
///
/// let set_of_integers_data_spec = SetSpecBuilder::new()
///     .set_element_spec(IntegerSpecBuilder::new()
///         .set_encoding(IntegerEncoding::Signed)
///         .set_storage(IntegerStorage::B64)
///        .build())
///    .set_storage(SetStorage::Unordered)
///    .build();
/// ```
pub struct SetSpecBuilder {
    element_spec: Option<Rc<DataSpec>>,
    storage: Option<SetStorage>,
}

impl SetSpecBuilder {
    /// Returns an initialized SetSpecBuilder.
    pub fn new() -> SetSpecBuilder {
        SetSpecBuilder {
            element_spec: None,
            storage: None,
        }
    }

    /// Sets the set's element specification.
    /// <p>
    /// Not setting an element specification will result in a set spec with no element specification and thus
    /// can only be used for comparison, not access.
    /// </p>
    pub fn set_element_spec(&mut self, element_spec: Rc<DataSpec>) -> &mut SetSpecBuilder {
        self.element_spec = Some(element_spec);
        self
    }

    /// Sets the set's storage type.
    /// <p>
    /// Not setting a storage type will result in a set spec with no storage type, which means the set will be unordered.
    /// </p>
    pub fn set_storage(&mut self, storage: SetStorage) -> &mut SetSpecBuilder {
        self.storage = Some(storage);
        self
    }

    /// Builds and returns an initialized set specification.
    ///
    /// # Panics
    ///
    /// If the set spec has a storage specified but no element specification.
    /// If the set spec is unordered and the element specification is not hashable.
    pub fn build(&self) -> Rc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<SetSpec, Set>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.element_spec.is_some() {
            let primitive_spec = Rc::new(SetSpec::new(&self.element_spec, self.storage));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
            let element_spec = self.element_spec.as_ref().unwrap();
            specification_level = element_spec.specification_level();
            if !element_spec.is_ordered() {
                panic!(
                    "SetSpecBuilder: Sets require element's that are ordered so that they compare and hash reliably."
                );
            }
        } else if self.storage.is_some() {
            panic!("SetSpecBuilder: storage is set but no element spec is set.");
        }

        Rc::new(DataSpec::new_primitive(
            Primitive::Set(primitive_def),
            specification_level,
        ))
    }
}
