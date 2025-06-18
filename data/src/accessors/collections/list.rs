use std::{
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use crate::{
    accessors::sequence::Sequence,
    adaptors::collection_adaptors::list_adaptor::ListAdaptor,
    primitive_def::Accessor,
    primitive_specs::list_spec::ListSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::{SpecCompatibility, SpecError},
    variable::Variable,
};

/// The List accessor provides access to lists, which are ordered collections of potentially
/// non-unique elements.
pub struct List {
    adaptor: Box<dyn ListAdaptor>,
}

impl List {
    /// Creates a new List accessor using the provided adaptor.
    pub fn new(adaptor: Box<dyn ListAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the list's specification.
    pub fn spec(&self) -> &Rc<ListSpec> {
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

    /// Returns a mutable reference to the value at the specified index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut Variable, ListError> {
        self.adaptor.get_mut(index)
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

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the list's elements as a sequence of referenced elements.
    /// This is useful for iterating over the elements in the list.
    pub fn elements(&self) -> Sequence {
        Sequence::new(self.adaptor.elements())
    }
}

impl SetEqualTo for List {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.spec().as_ref().check_compatible_with(other.spec())?;
        self.clear()?;
        for i in 0..other.len() {
            let value = other.get(i)?.try_clone()?;
            self.push(value)?;
        }
        Ok(())
    }
}

impl Accessor for List {}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            for i in 0..self.len() {
                if self.get(i) != other.get(i) {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl Eq for List {}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.len() == other.len() {
            for i in 0..self.len() {
                match (self.get(i), other.get(i)) {
                    (Ok(a), Ok(b)) => {
                        if a < b {
                            return std::cmp::Ordering::Less;
                        } else if a > b {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    (Err(_), Ok(_)) => return std::cmp::Ordering::Greater,
                    (Ok(_), Err(_)) => return std::cmp::Ordering::Less,
                    (Err(_), Err(_)) => continue, // Both errors, treat as equal
                }
            }
            std::cmp::Ordering::Equal
        } else {
            self.len().cmp(&other.len())
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len() {
            if i > 0 {
                write!(f, ", ")?;
            }
            match self.get(i) {
                Ok(var) => write!(f, "{}", var)?,
                Err(_) => write!(f, "<error>")?,
            }
        }
        write!(f, "]")
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List (length: {}", self.len())?;
        write!(f, ", value: [")?;
        for i in 0..self.len() {
            if i > 0 {
                write!(f, ", ")?;
            }
            match self.get(i) {
                Ok(var) => write!(f, "{}", var)?,
                Err(_) => write!(f, "<error>")?,
            }
        }
        write!(f, "])")
    }
}

impl Hash for List {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for i in 0..self.len() {
            if let Ok(var) = self.get(i) {
                var.hash(state);
            }
        }
    }
}

/// Errors that can occur when working with lists.
#[derive(Debug, PartialEq)]
pub enum ListError {
    /// A provider error.
    ProviderError(ProviderError),
    /// List index out of bounds.
    IndexOutOfBounds(usize, usize),
    /// An error indicating that popping from an empty list is not allowed.
    CannotPopOnEmpty,
    /// An error indicating that the list is fixed size and cannot be modified.
    FixedSizeViolation,
    /// An error that an operation would result in the list's fixed capacity being exceeded.
    FixedCapacityViolation(usize),
    /// An error indicating that the element specification is not compatible with the list's element specification.
    ElementSpecError(SpecError),
}

impl From<ProviderError> for ListError {
    fn from(err: ProviderError) -> Self {
        ListError::ProviderError(err)
    }
}

impl From<SpecError> for ListError {
    fn from(err: SpecError) -> Self {
        ListError::ElementSpecError(err)
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
            ListError::CannotPopOnEmpty => write!(f, "Cannot pop from an empty list"),
            ListError::FixedSizeViolation => {
                write!(f, "Cannot change the size of a fixed-size list")
            }
            ListError::FixedCapacityViolation(capacity) => {
                write!(f, "Operation would exceed fixed capacity of {}", capacity)
            }
            ListError::ElementSpecError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ListError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ListError::ProviderError(e) => Some(e),
            ListError::IndexOutOfBounds(..) => None,
            ListError::CannotPopOnEmpty => None,
            ListError::FixedSizeViolation => None,
            ListError::FixedCapacityViolation(..) => None,
            ListError::ElementSpecError(err) => Some(err),
        }
    }
}

impl From<ListError> for SetEqualToError {
    fn from(error: ListError) -> Self {
        SetEqualToError::ListError(error)
    }
}
