use std::{fmt::Display, rc::Rc};

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// The ordering of values in a set.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SetElementOrdering {
    /// Elements in the set are ordered, meaning the order of values matters.
    Ordered,
    /// Elements in the set are unordered, meaning the order of values does not matter.
    Unordered,
}

impl Display for SetElementOrdering {
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
    value_spec: Option<Rc<DataSpec>>,
    element_ordering: Option<SetElementOrdering>,
}

impl SetSpec {
    /// Returns an initialized set spec.
    /// Prefer to use the [`SetSpecBuilder`](crate::data_spec_builders::set_spec_builder::SetSpecBuilder) to create a set spec.
    pub fn new(
        value_spec: &Option<Rc<DataSpec>>,
        element_ordering: Option<SetElementOrdering>,
    ) -> SetSpec {
        SetSpec {
            value_spec: (value_spec.clone()),
            element_ordering: (element_ordering),
        }
    }

    /// Returns the set's value specification.
    /// If the set does not have an value specification, this will return None.
    /// If the set has an value specification, this will return Some(spec), where spec is the value specification.
    pub fn value_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.value_spec
    }
    /// Returns the set's element_ordering.
    /// If the set does not have a element_ordering specified, this will return None.
    pub fn element_ordering(&self) -> &Option<SetElementOrdering> {
        &self.element_ordering
    }
}

impl SpecCompatibility for SetSpec {
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
        match (self.element_ordering, required.element_ordering) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true, // required does not specify element_ordering, so we assume compatibility
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for SetSpec {
    fn is_ordered(&self) -> bool {
        // A set is ordered if it is ordered and its element spec is ordered.
        // If element ordering or element spec is None, we assume it is not ordered.
        self.element_ordering.as_ref() == Some(&SetElementOrdering::Ordered)
            && self
                .value_spec
                .as_ref()
                .is_some_and(|spec| spec.is_ordered())
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
            "Set {{ value_spec: {}, element_ordering: {} }}",
            self.value_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.element_ordering
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
