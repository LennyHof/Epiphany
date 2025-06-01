use std::sync::Arc;

use crate::{
    adaptors::collection_adaptors::list_adaptor::ListAdaptor, primitive_def::Accessor,
    primitive_specs::list_spec::ListSpec, provider_error::ProviderError, variable::Variable,
};

/// The List accessor provides access to lists, which are ordered collections of potentially
/// non-unique elements.
pub struct List {
    adaptor: Box<dyn ListAdaptor>,
}

impl Accessor for List {}

impl List {
    /// Creates a new List accessor using the provided adaptor.
    pub fn new(adaptor: Box<dyn ListAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the list's specification.
    pub fn spec(&self) -> &Arc<ListSpec> {
        self.adaptor.spec()
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.adaptor.len()
    }

    /// Returns the value at the specified index.
    pub fn get(&self, index: usize) -> Result<&Variable, ListError> {
        self.adaptor.get(index)
    }

    /// Sets the value at the specified index.
    pub fn set(&mut self, index: usize, value: Variable) -> Result<(), ListError> {
        self.adaptor.set(index, value)
    }

    /// Appends a value to the end of the list.
    pub fn push(&mut self, value: Variable) -> Result<(), ListError> {
        self.adaptor.push(value)
    }

    /// Removes and returns the last value from the list.
    pub fn pop(&mut self) -> Result<Option<Variable>, ListError> {
        self.adaptor.pop()
    }

    /// Inserts a value at the specified index.
    pub fn insert(&mut self, index: usize, value: Variable) -> Result<(), ListError> {
        self.adaptor.insert(index, value)
    }

    /// Removes the value at the specified index.
    pub fn remove(&mut self, index: usize) -> Result<(), ListError> {
        self.adaptor.remove(index)
    }

    /// Clears the list.
    pub fn clear(&mut self) -> Result<(), ListError> {
        self.adaptor.clear()
    }

    /// Returns the list's capacity.
    pub fn capacity(&self) -> usize {
        self.adaptor.capacity()
    }
}

/// Errors that can occur when working with lists.
#[derive(Debug, PartialEq)]

pub enum ListError {
    /// An provider error.
    ProviderError(ProviderError),
    /// List index out of bounds.
    IndexOutOfBounds(usize, usize),
    /// An error indicating that the list is fixed size and cannot be modified.
    /// This error is used when trying to modify a list that has a fixed size.
    FixedSizeList,
}

impl From<ProviderError> for ListError {
    fn from(err: ProviderError) -> Self {
        ListError::ProviderError(err)
    }
}

impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListError::ProviderError(err) => write!(f, "Provider Error: {}", err),
            ListError::IndexOutOfBounds(index, len) => write!(
                f,
                "Index {} out of bounds for list with length {}",
                index, len
            ),
            ListError::FixedSizeList => {
                write!(f, "Cannot change the size of a fixed-size list")
            }
        }
    }
}

impl std::error::Error for ListError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ListError::ProviderError(e) => Some(e),
            ListError::IndexOutOfBounds(..) => None,
            ListError::FixedSizeList => None,
        }
    }
}
