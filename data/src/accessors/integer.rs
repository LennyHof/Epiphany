use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use crate::{
    adaptors::integer_adaptor::IntegerAdaptor,
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_def::Accessor,
    primitive_specs::integer_spec::{IntegerEncoding, IntegerSpec, IntegerStorage},
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecCompatibility,
    variable::Variable,
};

/// The accessor for integers.
pub struct Integer {
    adaptor: Box<dyn IntegerAdaptor>,
}

impl Integer {
    /// Returns a new Integer that uses the provided adaptor.
    pub fn new(adaptor: Box<dyn IntegerAdaptor>) -> Integer {
        Integer { adaptor }
    }

    /// Returns the integer's specification
    pub fn spec(&self) -> &Rc<IntegerSpec> {
        self.adaptor.spec()
    }

    /// Sets the integer value as an unsigned 64 bit value.
    pub fn set_u64(&mut self, value: u64) -> Result<(), IntegerError> {
        self.adaptor.set_u64(value)
    }

    /// Returns the integer value as an unsigned 64 bit value.
    pub fn u64(&self) -> Result<u64, IntegerError> {
        self.adaptor.u64()
    }

    /// Sets the integer value as a signed 64 bit value.
    pub fn set_i64(&mut self, value: i64) -> Result<(), IntegerError> {
        self.adaptor.set_i64(value)
    }

    /// Returns the integer value as a signed 64 bit value.
    pub fn i64(&self) -> Result<i64, IntegerError> {
        self.adaptor.i64()
    }
}

impl SetEqualTo for Integer {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        self.spec().as_ref().check_compatible_with(other.spec())?;
        match self.spec().encoding() {
            Some(IntegerEncoding::Unsigned) => {
                let value = other.u64()?;
                self.set_u64(value)?;
                Ok(())
            }
            Some(IntegerEncoding::Signed) => {
                let value = other.i64()?;
                self.set_i64(value)?;
                Ok(())
            }
            None => panic!(), // should never happen, as the spec should always have an encoding
        }
    }
}

impl From<IntegerError> for SetEqualToError {
    fn from(error: IntegerError) -> Self {
        SetEqualToError::IntegerError(error)
    }
}

impl Accessor for Integer {}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        match (self.spec().encoding(), other.spec().encoding()) {
            (Some(IntegerEncoding::Unsigned), Some(IntegerEncoding::Unsigned)) => {
                self.u64().unwrap() == other.u64().unwrap()
            }
            (Some(IntegerEncoding::Signed), Some(IntegerEncoding::Signed)) => {
                self.i64().unwrap() == other.i64().unwrap()
            }
            _ => false,
        }
    }
}

impl Eq for Integer {}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.spec().encoding(), other.spec().encoding()) {
            (Some(IntegerEncoding::Unsigned), Some(IntegerEncoding::Unsigned)) => {
                self.u64().unwrap().cmp(&other.u64().unwrap())
            }
            (Some(IntegerEncoding::Signed), Some(IntegerEncoding::Signed)) => {
                self.i64().unwrap().cmp(&other.i64().unwrap())
            }
            _ => panic!("Cannot compare integers with different encodings"),
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.spec().encoding() {
            Some(IntegerEncoding::Unsigned) => write!(f, "{}", self.u64().unwrap_or(0)),
            Some(IntegerEncoding::Signed) => write!(f, "{}", self.i64().unwrap_or(0)),
            None => write!(f, "No encoding specified"),
        }
    }
}

impl Debug for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Hash for Integer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.spec().encoding() {
            Some(IntegerEncoding::Unsigned) => self.u64().unwrap_or(0).hash(state),
            Some(IntegerEncoding::Signed) => self.i64().unwrap_or(0).hash(state),
            None => 0.hash(state), // No encoding, hash as 0
        }
    }
}

impl TryFrom<Variable> for u8 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result as u8)
    }
}

impl TryFrom<Variable> for i8 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result as i8)
    }
}
impl TryFrom<Variable> for u16 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result as u16)
    }
}
impl TryFrom<Variable> for i16 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result as i16)
    }
}
impl TryFrom<Variable> for u32 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result as u32)
    }
}
impl TryFrom<Variable> for i32 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result as i32)
    }
}
impl TryFrom<Variable> for u64 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result)
    }
}
impl TryFrom<Variable> for i64 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result)
    }
}
// Convert all integer types from a Variable
impl TryFrom<u8> for Variable {
    type Error = IntegerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B8)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_u64(value as u64)?;
        Ok(var)
    }
}
impl TryFrom<i8> for Variable {
    type Error = IntegerError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B8)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_i64(value as i64)?;
        Ok(var)
    }
}
impl TryFrom<u16> for Variable {
    type Error = IntegerError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B16)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_u64(value as u64)?;
        Ok(var)
    }
}
impl TryFrom<i16> for Variable {
    type Error = IntegerError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B16)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_i64(value as i64)?;
        Ok(var)
    }
}
impl TryFrom<u32> for Variable {
    type Error = IntegerError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B32)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_u64(value as u64)?;
        Ok(var)
    }
}
impl TryFrom<i32> for Variable {
    type Error = IntegerError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B32)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_i64(value as i64)?;
        Ok(var)
    }
}
impl TryFrom<u64> for Variable {
    type Error = IntegerError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B64)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_u64(value)?;
        Ok(var)
    }
}
impl TryFrom<i64> for Variable {
    type Error = IntegerError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B64)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = Variable::new(&spec);
        let int = var.integer_mut();
        int.set_i64(value)?;
        Ok(var)
    }
}

/// An integer error.
#[derive(Debug, PartialEq)]
pub enum IntegerError {
    /// A provider error
    ProviderError(ProviderError),
    /// An overflow error
    Overflow(String),
}

impl Error for IntegerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IntegerError::ProviderError(e) => Some(e),
            IntegerError::Overflow(_) => None,
        }
    }
}

impl Display for IntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerError::ProviderError(err) => write!(f, "{:?}", err),
            IntegerError::Overflow(msg) => write!(f, "Overflow Error: {}", msg),
        }
    }
}

impl From<String> for IntegerError {
    fn from(value: String) -> Self {
        IntegerError::ProviderError(ProviderError::General(value))
    }
}

impl From<ProviderError> for IntegerError {
    fn from(value: ProviderError) -> Self {
        IntegerError::ProviderError(value)
    }
}

impl From<IntegerError> for String {
    fn from(value: IntegerError) -> Self {
        value.to_string()
    }
}
