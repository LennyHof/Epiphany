use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// IntegerEncoding defines an enumeration that captures the supported encoding
/// characteristics for integers and unsigned integers.
/// <p>
/// You set an IntegerEncoding option in a DataSpec for a variable or attribute
/// that is to accept an integer as its value.
/// An IntegerEncoding option describes how integer values are translated to bits or bytes
/// in storage.
/// </p>
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntegerEncoding {
    /// Two's complement integer encoding.
    Signed,
    /// Binary encoding.
    Unsigned,
}

impl Display for IntegerEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Signed => "Signed".to_string(),
                Self::Unsigned => "Unsigned".to_string(),
            }
        )
    }
}

/// IntegerStorage defines an enumeration that captures the supported storage
/// characteristics for integers and unsigned integers.
/// <p>
/// These values correspond to the size in bytes for each type; do not change
/// as this assumption is used in other places.
/// </p>
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntegerStorage {
    /// One byte per integer.
    B8 = 1,
    /// Two bytes per integer.
    B16 = 2,
    /// Four bytes per integer.
    B32 = 4,
    /// Eight bytes per integer.
    B64 = 8,
}

impl Display for IntegerStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::B8 => "B8".to_string(),
                Self::B16 => "B16".to_string(),
                Self::B32 => "B32".to_string(),
                Self::B64 => "B64".to_string(),
            }
        )
    }
}

/// A primitive spec for integers.
#[derive(Debug, PartialEq)]
pub struct IntegerSpec {
    encoding: Option<IntegerEncoding>,
    storage: Option<IntegerStorage>,
}

impl IntegerSpec {
    /// Returns an initialized integer spec.
    /// Prefer to use the [`IntegerSpecBuilder`](crate::data_spec_builders::integer_spec_builder::IntegerSpecBuilder) to create an interger spec.
    pub fn new(encoding: Option<IntegerEncoding>, storage: Option<IntegerStorage>) -> IntegerSpec {
        IntegerSpec {
            encoding: (encoding),
            storage: (storage),
        }
    }

    /// Returns the integer's encoding.
    pub fn encoding(&self) -> &Option<IntegerEncoding> {
        &self.encoding
    }

    /// Returns the integer's storage.
    pub fn storage(&self) -> &Option<IntegerStorage> {
        &self.storage
    }

    /// Returns true if the integer is signed; false otherwise.
    /// Panics if encoding has not been specified.
    pub fn is_signed(&self) -> bool {
        self.encoding.unwrap() == IntegerEncoding::Signed
    }
}

impl SpecCompatibility for IntegerSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        if !match (self.encoding, required.encoding) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        } {
            return false;
        }
        match (self.storage, required.storage) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true,
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for IntegerSpec {
    fn is_ordered(&self) -> bool {
        // Integers are ordered.
        true
    }
}

impl PrimitiveSpec for IntegerSpec {}

impl Display for IntegerSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Integer {{ encoding: {}, storage: {} }}",
            self.encoding.map_or("None".to_string(), |e| e.to_string()),
            self.storage.map_or("None".to_string(), |s| s.to_string())
        )
    }
}
