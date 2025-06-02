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
#[derive(Debug, PartialEq)]
pub struct ListSpec {
    element_spec: Option<Arc<DataSpec>>,
    storage: Option<ListStorage>,
}

impl ListSpec {
    /// Returns an initialized list spec.
    /// Prefer to use the [`ListSpecBuilder`](crate::data_spec_builders::list_spec_builder::ListSpecBuilder) to create a list spec.
    pub fn new(element_spec: &Option<Arc<DataSpec>>, storage: &Option<ListStorage>) -> ListSpec {
        ListSpec {
            element_spec: element_spec.clone(),
            storage: *storage,
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
    /// Returns if this list spec is compatible with the required spec.
    pub fn is_compatible_with(&self, required: &Self) -> bool {
        if self.element_spec.is_some() && required.element_spec.is_some() {
            if let Some(element_spec) = self.element_spec.as_ref() {
                if let Some(required_element_spec) = required.element_spec.as_ref() {
                    if !element_spec.is_compatible_with(required_element_spec) {
                        return false;
                    }
                }
            }
        } else if self.element_spec.is_none() && required.element_spec.is_some() {
            return false;
        }

        match (self.storage, required.storage) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true, // required does not specify storage, so we assume compatibility
            (None, Some(_)) => false,
        }
    }
}

impl PrimitiveSpec for ListSpec {}

impl Display for ListSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "List {{ element_spec: {}, storage: {} }}",
            self.element_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.storage
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
