use crate::{
    accessors::collections::list::List,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::list_spec::{ListSpec, ListStorage},
};
use core::panic;
use std::sync::Arc;

/// Builder for data specifications for lists.
///
/// # Examples
///
/// Create a data specification for a variable list of signed 64 bit integers:
/// ```rust
/// use data::data_spec_builders::{list_spec_builder::ListSpecBuilder, integer_spec_builder::IntegerSpecBuilder};
/// use data::primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage};
/// use data::primitive_specs::list_spec::ListStorage;
///
/// let list_of_integers_data_spec = ListSpecBuilder::new()
///     .set_element_spec(IntegerSpecBuilder::new()
///       .set_encoding(IntegerEncoding::Signed)
///       .set_storage(IntegerStorage::B64)
///       .build())
///     .set_storage(ListStorage::VariableSize)
///   .build();
/// ```
///
pub struct ListSpecBuilder {
    element_spec: Option<Arc<DataSpec>>,
    storage: Option<ListStorage>,
}

impl ListSpecBuilder {
    /// Returns an initialized list spec builder.
    pub fn new() -> ListSpecBuilder {
        ListSpecBuilder {
            element_spec: None,
            storage: None,
        }
    }

    /// Sets the list's element specification.
    /// <p>
    /// Not setting an element specification will result in a list spec with no element specification and thus
    /// can only be used for comparison, not access.
    /// </p>
    pub fn set_element_spec(&mut self, element_spec: Arc<DataSpec>) -> &mut ListSpecBuilder {
        self.element_spec = Some(element_spec.clone());
        self
    }

    /// Sets the list's storage type.
    /// <p>
    /// Not setting a storage type will result in a list spec with no storage type.
    /// </p>    
    /// <p>
    /// Setting a storage type but not setting an element specification will result in build panicing.
    /// </p>
    pub fn set_storage(&mut self, storage: ListStorage) -> &mut ListSpecBuilder {
        match storage {
            ListStorage::FixedSize(size) => {
                if size == 0 {
                    panic!("ListSpecBuilder: FixedSize cannot be zero.");
                }
            }
            ListStorage::FixedCapacity(capacity) => {
                if capacity == 0 {
                    panic!("ListSpecBuilder: FixedCapacity cannot be zero.");
                }
            }
            ListStorage::InitialCapacity(capacity) => {
                if capacity == 0 {
                    panic!("ListSpecBuilder: InitialCapacity cannot be zero.");
                }
            }
            ListStorage::VariableSize => {}
        }
        self.storage = Some(storage);
        self
    }

    /// Builds and returns an initialized data specification.
    ///
    /// # Panics
    ///
    /// If the list spec has a fixed size or fixed capacity but no element specification.
    pub fn build(&self) -> Arc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<ListSpec, List>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.element_spec.is_some() {
            let primitive_spec = Arc::new(ListSpec::new(&self.element_spec, &self.storage));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
            specification_level = self.element_spec.as_ref().unwrap().specification_level();
        } else {
            if self.storage.is_some() {
                panic!("ListSpecBuilder: list storage is set but no element spec is set.");
            }
        }
        Arc::new(DataSpec::new_primitive(
            Primitive::List(primitive_def),
            specification_level,
        ))
    }
}
