use std::rc::Rc;

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for sequences.
#[derive(Debug, PartialEq)]
pub struct SequenceSpec {
    value_spec: Option<Rc<DataSpec>>,
}

impl SequenceSpec {
    /// Creates a new sequence spec.
    pub fn new(value_spec: &Option<Rc<DataSpec>>) -> Self {
        Self {
            value_spec: value_spec.clone(),
        }
    }

    /// Returns the value specification of the sequence.
    pub fn value_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.value_spec
    }
}

impl SpecCompatibility for SequenceSpec {
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
        true
    }
}

impl IsOrdered for SequenceSpec {
    fn is_ordered(&self) -> bool {
        false // Sequences are not ordered, as they cannot be a collection element.
    }
}

impl PrimitiveSpec for SequenceSpec {}

impl std::fmt::Display for SequenceSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sequence {{ value_spec: {} }}",
            self.value_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
