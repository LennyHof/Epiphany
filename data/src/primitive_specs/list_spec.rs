use std::{fmt::Display, sync::Arc};

use crate::{data_spec::DataSpec, primitive_def::PrimitiveSpec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The storage type for lists.
pub enum ListStorage {
    /// The list is stored as a fixed-size list.
    FixedSize(u64),
    /// The list is stored as a fixed-capacity list, where size can grow upto but not beyond the specified capacity.
    FixedCapacity(u64),
    /// The list is stored as a list with a specified initial capacity.
    /// This is useful for performance optimization when a reasonable mimimim size of the list is known in advance.
    InitialCapacity(u64),
    /// The list is stored as a variable-size list.
    VariableSize,
}

impl Display for ListStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::FixedSize(size) => format!("FixedSize({})", size),
                Self::FixedCapacity(capacity) => format!("FixedCapacity({})", capacity),
                Self::InitialCapacity(capacity) => format!("InitialCapacity({})", capacity),
                Self::VariableSize => "VariableSize".to_string(),
            }
        )
    }
}

/// A primitive spec for lists.
pub struct ListSpec {
    element_spec: Option<Arc<DataSpec>>,
    storage: Option<ListStorage>,
}

impl ListSpec {
    /// Returns an initialized list spec.
    /// Prefer to use the [`ListSpecBuilder`](crate::spec_builders::list_spec_builder::ListSpecBuilder) to create a list spec.
    pub fn new(element_spec: &Option<Arc<DataSpec>>, storage: &Option<ListStorage>) -> ListSpec {
        ListSpec {
            element_spec: (element_spec.clone()),
            storage: (storage.clone()),
        }
    }

    /// Returns the list' element specification.
    /// If the list does not have an element specification, this will return None.
    /// If the list has an element specification, this will return Some(spec), where spec is the element specification.
    pub fn element_spec(&self) -> &Option<Arc<DataSpec>> {
        &self.element_spec
    }

    /// Returns the list's storage type.
    /// If the list does not have a storage type, this will return None.
    /// If the list has a storage type, this will return Some(storage), where storage is the storage type.
    pub fn storage(&self) -> &Option<ListStorage> {
        &self.storage
    }
}

impl PrimitiveSpec for ListSpec {}
