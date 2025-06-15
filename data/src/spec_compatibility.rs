use std::fmt::Display;
/// A trait for data specifications that can check compatibility with other specifications.
pub trait SpecCompatibility: Display {
    /// Checks if this spec is compatible with the required spec.
    fn is_compatible_with(&self, required: &Self) -> bool;

    /// Checks if this spec is compatible with the required spec.
    /// If the specs are not compatible, it returns an error with a message indicating the incompatibility.
    fn check_compatible_with(&self, required: &Self) -> Result<(), SpecError> {
        if !self.is_compatible_with(required) {
            return Err(SpecError::IncompatibleSpec(
                self.to_string(),
                required.to_string(),
            ));
        }
        Ok(())
    }
}

/// An error that can occur when working with data specifications.
#[derive(Debug, PartialEq)]
pub enum SpecError {
    /// An error indicating that the data specification is not compatible with the required specification.
    IncompatibleSpec(String, String),
}

impl std::error::Error for SpecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for SpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecError::IncompatibleSpec(provided, required) => write!(
                f,
                "Incompatible value: provided: {}, required: {}",
                provided, required
            ),
        }
    }
}
