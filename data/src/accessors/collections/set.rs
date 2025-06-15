use std::rc::Rc;

use crate::{
    adaptors::collection_adaptors::set_adaptor::SetAdaptor,
    primitive_def::Accessor,
    primitive_specs::set_spec::SetSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::{SpecCompatibility, SpecError},
    variable::Variable,
};

/// The Set accessor provides access to sets, which are ordered or unordered collections of
/// unique elements.
pub struct Set {
    // The adaptor for the set.
    adaptor: Box<dyn SetAdaptor>,
}

impl Set {
    /// Creates a new `Set` accessor.
    pub fn new(adaptor: Box<dyn SetAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the set's specification.
    pub fn spec(&self) -> &Rc<SetSpec> {
        self.adaptor.spec()
    }

    /// Returns the length of the set.
    pub fn len(&self) -> usize {
        self.adaptor.len()
    }

    /// Checks if the set contains a value.
    pub fn contains(&self, value: &Variable) -> Result<bool, SetError> {
        self.adaptor.contains(value)
    }

    /// Adds a value to the set.
    pub fn add(&mut self, value: Variable) -> Result<(), SetError> {
        self.adaptor.add(value)
    }

    /// Removes a value from the set.
    pub fn remove(&mut self, value: &Variable) -> Result<bool, SetError> {
        self.adaptor.remove(value)
    }

    /// Clears the set.
    pub fn clear(&mut self) -> Result<(), SetError> {
        self.adaptor.clear()
    }
}

impl SetEqualTo for Set {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.spec().as_ref().check_compatible_with(other.spec())?;
        self.clear()?;
        todo!("Implement set_equal_to for Set");
    }
}

impl Accessor for Set {}

// impl PartialEq for Set {
//     fn eq(&self, other: &Self) -> bool {
//         if self.len() == other.len() {
//             for value in self.adaptor.spec().element_spec().as_ref().unwrap().iter() {
//                 if !other.contains(value).unwrap_or(false) {
//                     return false;
//                 }
//             }
//             true
//         } else {
//             false
//         }
//     }
// }

/// Errors that can occur when working with sets.
#[derive(Debug, PartialEq)]
pub enum SetError {
    /// A provider error.
    ProviderError(ProviderError),
    /// An error indicating that the element specification is not compatible with the list's element specification.
    ElementSpecError(SpecError),
}

impl From<ProviderError> for SetError {
    fn from(error: ProviderError) -> Self {
        SetError::ProviderError(error)
    }
}

impl std::fmt::Display for SetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetError::ProviderError(e) => write!(f, "Provider error: {}", e),
            SetError::ElementSpecError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SetError::ProviderError(e) => Some(e),
            SetError::ElementSpecError(e) => Some(e),
        }
    }
}

impl From<SetError> for SetEqualToError {
    fn from(error: SetError) -> Self {
        SetEqualToError::SetError(error)
    }
}

impl From<SpecError> for SetError {
    fn from(error: SpecError) -> Self {
        SetError::ElementSpecError(error)
    }
}
