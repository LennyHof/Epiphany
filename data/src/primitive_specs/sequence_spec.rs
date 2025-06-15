use std::rc::Rc;

use crate::{
    data_spec::DataSpec, primitive_def::PrimitiveSpec, spec_compatibility::SpecCompatibility,
};

/// A primitive spec for sequences.
#[derive(Debug, PartialEq)]
pub struct SequenceSpec {
    element_spec: Option<Rc<DataSpec>>,
}

impl SequenceSpec {
    /// Creates a new sequence spec.
    pub fn new(element_spec: &Option<Rc<DataSpec>>) -> Self {
        Self {
            element_spec: element_spec.clone(),
        }
    }

    /// Returns the element specification of the sequence.
    pub fn element_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.element_spec
    }
}

impl SpecCompatibility for SequenceSpec {
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
        true
    }
}

impl PrimitiveSpec for SequenceSpec {}

impl std::fmt::Display for SequenceSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sequence {{ element_spec: {} }}",
            self.element_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
