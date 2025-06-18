use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    adaptors::boolean_adaptor::BooleanAdaptor,
    data_spec_builders::boolean_spec_builder::BooleanSpecBuilder,
    primitive_def::Accessor,
    primitive_specs::boolean_spec::BooleanSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecCompatibility,
    variable::Variable,
};

/// The accessor for booleans.
pub struct Boolean {
    adaptor: Box<dyn BooleanAdaptor>,
}

impl Boolean {
    /// Creates a new boolean accessor.
    pub fn new(adaptor: Box<dyn BooleanAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the boolean's specification.
    pub fn spec(&self) -> &BooleanSpec {
        self.adaptor.spec()
    }

    /// Returns the value of the boolean.
    pub fn boolean(&self) -> Result<bool, ProviderError> {
        self.adaptor.boolean()
    }

    /// Sets the value of the boolean.
    pub fn set_boolean(&mut self, value: bool) -> Result<(), ProviderError> {
        self.adaptor.set_boolean(value)
    }
}

impl SetEqualTo for Boolean {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.spec().check_compatible_with(other.spec())?;
        let value = other.boolean()?;
        self.set_boolean(value)?;
        Ok(())
    }
}

impl Accessor for Boolean {}

impl PartialEq for Boolean {
    fn eq(&self, other: &Self) -> bool {
        match (self.adaptor.boolean(), other.adaptor.boolean()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Boolean {}

impl PartialOrd for Boolean {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Boolean {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.boolean()
            .unwrap_or(false)
            .cmp(&other.boolean().unwrap_or(false))
    }
}

impl Hash for Boolean {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.boolean().unwrap_or(false).hash(state);
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.boolean().unwrap_or(false))
    }
}

impl Debug for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl TryFrom<bool> for Variable {
    type Error = ProviderError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        let spec = BooleanSpecBuilder::new().build();
        let mut var = Variable::new(&spec);
        let boolean = var.boolean_mut();
        boolean.set_boolean(value)?;
        Ok(var)
    }
}

impl TryFrom<Variable> for bool {
    type Error = ProviderError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let boolean = value.boolean();
        let result = boolean.boolean()?;
        Ok(result)
    }
}
