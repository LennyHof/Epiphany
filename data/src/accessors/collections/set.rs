use std::fmt::{Debug, Display};
use std::{hash::Hash, rc::Rc};

use crate::{
    adaptors::collection_adaptors::set_adaptor::SetAdaptor,
    primitive_def::Accessor,
    primitive_specs::set_spec::{SetElementOrdering, SetSpec},
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::{SpecCompatibility, SpecError},
    variable::Variable,
};

/// The Set accessor provides access to sets, which are ordered or unordered collections of
/// unique values.
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

    /// Checks if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.adaptor.is_empty()
    }

    /// Returns the length of the set.
    pub fn len(&self) -> usize {
        self.adaptor.len()
    }

    /// Checks if the set contains a value.
    pub fn contains(&self, value: &Variable) -> Result<bool, SetError> {
        self.adaptor.contains(value)
    }

    /// Adds a value to the set. Returns `true` if the value was added, `false` if it was already present.
    pub fn insert(&mut self, value: Variable) -> Result<bool, SetError> {
        self.adaptor.insert(value)
    }

    /// Removes a value from the set.
    pub fn remove(&mut self, value: &Variable) -> Result<bool, SetError> {
        self.adaptor.remove(value)
    }

    /// Clears the set.
    pub fn clear(&mut self) -> Result<(), SetError> {
        self.adaptor.clear()
    }

    /// Returns an iterator for the set's values.
    pub fn iter<'a>(&'a self) -> Box<dyn SetIter<'a> + 'a> {
        self.adaptor.values()
    }
}

/// An iterator for set values.
pub trait SetIter<'a>: Iterator<Item = Result<&'a Variable, SetError>> {}

impl<'a> IntoIterator for &'a Set {
    type Item = Result<&'a Variable, SetError>;
    type IntoIter = Box<dyn SetIter<'a> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Set {
    type Item = Result<&'a Variable, SetError>;
    type IntoIter = Box<dyn SetIter<'a> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        // We can use the same iterator for mutable access, as it does not change the set's structure.
        self.iter()
    }
}

impl SetEqualTo for Set {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.spec().as_ref().check_compatible_with(other.spec())?;
        self.clear()?;
        for element_result in other {
            match element_result {
                Ok(element) => {
                    let cloned_element = element.try_clone()?;
                    self.insert(cloned_element)?;
                }
                Err(e) => return Err(SetEqualToError::from(e)),
            }
        }
        Ok(())
    }
}

impl Accessor for Set {}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for element_result in self {
            match element_result {
                Ok(element) => {
                    if !other.contains(element).unwrap_or(false) {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Set {}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match *self.spec().as_ref().element_ordering() {
            Some(SetElementOrdering::Ordered) => {
                if self.len() != other.len() {
                    return self.len().cmp(&other.len());
                }
                for (a, b) in self.iter().zip(other.iter()) {
                    match (a, b) {
                        (Ok(a), Ok(b)) => match a.cmp(b) {
                            std::cmp::Ordering::Equal => continue,
                            ord => return ord,
                        },
                        (Err(_), Ok(_)) => return std::cmp::Ordering::Greater,
                        (Ok(_), Err(_)) => return std::cmp::Ordering::Less,
                        (Err(_), Err(_)) => continue, // Both are errors, continue to next
                    }
                }
                std::cmp::Ordering::Equal
            }
            _ => {
                panic!(
                    "Comparison of unordered sets is not supported. Use a different method to compare sets."
                );
            }
        }
    }
}

impl Hash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.spec().as_ref().element_ordering() {
            Some(SetElementOrdering::Ordered) => {
                // If the set is ordered, we can hash the values in order.
                self.len().hash(state);
                for element in self.iter().flatten() {
                    element.hash(state);
                }
            }
            _ => {
                panic!(
                    "Hashing of unordered sets is not supported. Use a different method to compare sets."
                );
            }
        }
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Set {{")?;
        let mut first = true;
        for element_result in self {
            if let Ok(element) = element_result {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", element)?;
                first = false;
            } else {
                write!(f, "<error>")?;
            }
        }
        write!(f, "}}")
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// Errors that can occur when working with sets.
#[derive(Debug, PartialEq)]
pub enum SetError {
    /// A provider error.
    ProviderError(ProviderError),
    /// An error indicating that the value specification is not compatible with the list's value specification.
    ValueSpecError(SpecError),
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
            SetError::ValueSpecError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SetError::ProviderError(e) => Some(e),
            SetError::ValueSpecError(e) => Some(e),
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
        SetError::ValueSpecError(error)
    }
}
