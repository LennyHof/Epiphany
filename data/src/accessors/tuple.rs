use std::{
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use crate::{
    adaptors::tuple_adaptor::TupleAdaptor,
    primitive_def::Accessor,
    primitive_specs::tuple_spec::TupleSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::{SpecCompatibility, SpecError},
    variable::Variable,
};

/// An accessor for time values.
pub struct Tuple {
    /// The adaptor for the tuple.
    adaptor: Box<dyn TupleAdaptor>,
}

impl Tuple {
    /// Creates a new `Tuple` accessor using the provided adaptor.
    pub fn new(adaptor: Box<dyn TupleAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the tuple's specification.
    pub fn spec(&self) -> &Rc<TupleSpec> {
        self.adaptor.spec()
    }

    /// Returns the number of elements in the tuple.
    pub fn len(&self) -> usize {
        self.adaptor.len()
    }

    /// Returns the value at the specified index.
    pub fn get(&self, index: usize) -> Result<&Variable, TupleError> {
        self.adaptor.get(index)
    }

    /// Returns a mutable reference to the value at the specified index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut Variable, TupleError> {
        self.adaptor.get_mut(index)
    }

    /// Sets the value at the specified index.
    pub fn set(&mut self, index: usize, value: Variable) -> Result<(), TupleError> {
        self.adaptor.set(index, value)
    }
}

impl SetEqualTo for Tuple {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.spec().as_ref().check_compatible_with(other.spec())?;
        for i in 0..other.len() {
            let value = other.get(i)?.try_clone()?;
            self.set(i, value)?;
        }
        Ok(())
    }
}

impl Accessor for Tuple {}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        if self.spec() != other.spec() {
            return false;
        }
        for i in 0..self.len() {
            if self.get(i).ok() != other.get(i).ok() {
                return false;
            }
        }
        true
    }
}

impl Eq for Tuple {}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.spec() != other.spec() {
            return std::cmp::Ordering::Equal; // or handle as needed
        }
        for i in 0..self.len() {
            let lhs = self.get(i).ok();
            let rhs = other.get(i).ok();
            match lhs.cmp(&rhs) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl Hash for Tuple {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..self.len() {
            if let Ok(value) = self.get(i) {
                value.hash(state);
            }
        }
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tuple {{")?;
        for i in 0..self.len() {
            if i > 0 {
                write!(f, ", ")?;
            }
            match self.get(i) {
                Ok(value) => write!(f, "{}", value)?,
                Err(_) => write!(f, "{}: Error", i)?,
            }
        }
        write!(f, "}}")
    }
}

impl Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// Errors that can occur when working with tuples.
#[derive(Debug, PartialEq)]
pub enum TupleError {
    /// A provider error.
    ProviderError(ProviderError),
    /// Tuple index out of bounds.
    IndexOutOfBounds(usize, usize),
    /// An error indicating that the value specification is not compatible with the tuple's value specification.
    ValueSpecError(SpecError),
}

impl From<ProviderError> for TupleError {
    fn from(err: ProviderError) -> Self {
        TupleError::ProviderError(err)
    }
}

impl From<SpecError> for TupleError {
    fn from(err: SpecError) -> Self {
        TupleError::ValueSpecError(err)
    }
}

impl std::fmt::Display for TupleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TupleError::ProviderError(err) => write!(f, "Provider Error: {}", err),
            TupleError::IndexOutOfBounds(index, len) => write!(
                f,
                "Index {} out of bounds for tuple with length {}",
                index, len
            ),
            TupleError::ValueSpecError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for TupleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TupleError::ProviderError(e) => Some(e),
            TupleError::IndexOutOfBounds(..) => None,
            TupleError::ValueSpecError(err) => Some(err),
        }
    }
}

impl From<TupleError> for SetEqualToError {
    fn from(error: TupleError) -> Self {
        SetEqualToError::TupleError(error)
    }
}
