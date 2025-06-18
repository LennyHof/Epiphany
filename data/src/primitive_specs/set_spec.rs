use std::{fmt::Display, rc::Rc};

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// The storage type of a set.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SetStorage {
    /// An ordered set, where the order of elements matters.
    Ordered,
    /// An unordered set, where the order of elements does not matter.
    Unordered,
}

impl Display for SetStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Ordered => "Ordered".to_string(),
                Self::Unordered => "Unordered".to_string(),
            }
        )
    }
}

/// A primitive spec for sets.
#[derive(Debug, PartialEq)]
pub struct SetSpec {
    element_spec: Option<Rc<DataSpec>>,
    storage: Option<SetStorage>,
}

impl SetSpec {
    /// Returns an initialized set spec.
    /// Prefer to use the [`SetSpecBuilder`](crate::data_spec_builders::set_spec_builder::SetSpecBuilder) to create a set spec.
    pub fn new(element_spec: &Option<Rc<DataSpec>>, storage: Option<SetStorage>) -> SetSpec {
        SetSpec {
            element_spec: (element_spec.clone()),
            storage: (storage),
        }
    }

    /// Returns the set's element specification.
    /// If the set does not have an element specification, this will return None.
    /// If the set has an element specification, this will return Some(spec), where spec is the element specification.
    pub fn element_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.element_spec
    }
    /// Returns the set's storage type.
    /// If the set does not have a storage type, this will return None.
    pub fn storage(&self) -> &Option<SetStorage> {
        &self.storage
    }
}

impl SpecCompatibility for SetSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
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

impl IsOrdered for SetSpec {
    fn is_ordered(&self) -> bool {
        // A set is ordered if it is ordered and its element spec is ordered.
        // If storage or element spec is None, we assume it is not ordered.
        self.storage.as_ref() == Some(&SetStorage::Ordered)
            && self
                .element_spec
                .as_ref()
                .map_or(false, |spec| spec.is_ordered())
    }
}

impl PrimitiveSpec for SetSpec {}

impl Default for SetSpec {
    fn default() -> Self {
        Self::new(&None, None)
    }
}

impl std::fmt::Display for SetSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Set {{ element_spec: {}, storage: {} }}",
            self.element_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.storage
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
