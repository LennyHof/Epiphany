use std::rc::Rc;

use crate::{
    adaptors::sequence_adaptor::SequenceAdaptor,
    primitive_def::Accessor,
    primitive_specs::sequence_spec::SequenceSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecError,
    variable::Variable,
};

/// An accessor for sequences.
pub struct Sequence {
    adaptor: Box<dyn SequenceAdaptor>,
}

impl Sequence {
    /// Creates a new sequence accessor using the provided adaptor.
    pub fn new(adaptor: Box<dyn SequenceAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the sequence's specification.
    pub fn spec(&self) -> &Rc<SequenceSpec> {
        self.adaptor.spec()
    }

    /// Returns a sequence iterator.
    pub fn iter<'a>(&'a self) -> Box<dyn SequenceIter<'a> + 'a> {
        self.adaptor.iter()
    }

    /// Sets the sequence equal to another sequence.
    ///
    /// Returns an error if the sequence is read-only.
    pub fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.adaptor.set_equal_to(other)?;
        Ok(())
    }
}

/// An iterator for sequences.
pub trait SequenceIter<'a>: Iterator<Item = Result<&'a Variable, SequenceError>> {}

impl<'a> IntoIterator for &'a Sequence {
    type Item = Result<&'a Variable, SequenceError>;
    type IntoIter = Box<dyn SequenceIter<'a> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Sequence {
    type Item = Result<&'a Variable, SequenceError>;
    type IntoIter = Box<dyn SequenceIter<'a> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl SetEqualTo for Sequence {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.set_equal_to(other)?;
        Ok(())
    }
}

impl Accessor for Sequence {}

/// Errors that can occur when working with sequences.
#[derive(Debug, PartialEq)]
pub enum SequenceError {
    /// A provider error.
    ProviderError(ProviderError),
    /// An error indicating that the sequence is read-only and cannot be modified.
    ReadOnlyError,
    /// An error indicating that another sequence specification is not compatible with this sequence's specification.
    SpecError(SpecError),
}

impl From<ProviderError> for SequenceError {
    fn from(error: ProviderError) -> Self {
        SequenceError::ProviderError(error)
    }
}

impl std::fmt::Display for SequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceError::ProviderError(err) => write!(f, "Provider error: {}", err),
            SequenceError::ReadOnlyError => {
                write!(f, "Sequences are read only and thus cannot be modified.")
            }
            SequenceError::SpecError(err) => write!(f, "Specification error: {}", err),
        }
    }
}

impl std::error::Error for SequenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SequenceError::ProviderError(e) => Some(e),
            SequenceError::ReadOnlyError => None,
            SequenceError::SpecError(e) => Some(e),
        }
    }
}

impl From<SequenceError> for SetEqualToError {
    fn from(error: SequenceError) -> Self {
        SetEqualToError::SequenceError(error)
    }
}
