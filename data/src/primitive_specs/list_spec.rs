use std::{fmt::Display, rc::Rc};

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

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
    value_spec: Option<Rc<DataSpec>>,
    storage: Option<ListStorage>,
}

impl ListSpec {
    /// Returns an initialized list spec.
    pub(crate) fn new(
        value_spec: &Option<Rc<DataSpec>>,
        storage: &Option<ListStorage>,
    ) -> ListSpec {
        ListSpec {
            value_spec: value_spec.clone(),
            storage: *storage,
        }
    }

    /// Returns the list' value specification.
    /// If the list does not have an value specification, this will return None.
    /// If the list has an value specification, this will return Some(spec), where spec is the value specification.
    pub fn value_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.value_spec
    }

    /// Returns the list's storage type.
    /// If the list does not have a storage type, this will return None.
    /// If the list has a storage type, this will return Some(storage), where storage is the storage type.
    pub fn storage(&self) -> &Option<ListStorage> {
        &self.storage
    }
}

impl SpecCompatibility for ListSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        if self.value_spec.is_some() && required.value_spec.is_some() {
            if let Some(value_spec) = self.value_spec.as_ref() {
                if let Some(required_value_spec) = required.value_spec.as_ref() {
                    if !value_spec.is_compatible_with(required_value_spec) {
                        return false;
                    }
                }
            }
        } else if self.value_spec.is_none() && required.value_spec.is_some() {
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

impl IsOrdered for ListSpec {
    fn is_ordered(&self) -> bool {
        // A list spec is ordered if its element spec is ordered.
        self.value_spec
            .as_ref()
            .is_none_or(|spec| spec.is_ordered())
    }
}

impl PrimitiveSpec for ListSpec {}

impl Display for ListSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "List {{ value_spec: {}, storage: {} }}",
            self.value_spec
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
