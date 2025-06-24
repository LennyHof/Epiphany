use std::fmt::Display;

use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// StringEncoding defines an enumeration that captures the supported encoding
/// schemes for strings.
/// <p>
/// You set a StringEncoding option in a DataSpec for a variable or attribute
/// that is to accept a String as its value.
/// A StringEncoding option describes how the characters of the accessed string are
/// encoded. More specifically, each option constrains the number, size, and type of the
/// code units used for representing the characters.
/// </p>
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StringEncoding {
    /// Byte (8-bit) encoding.
    /// <p>
    /// Characters are represented as numeric values according to a standard such as ASCII
    /// or ISO Latin-1.  Byte,
    /// </p>
    Byte,
    /// Unicode UTF-8 encoding.
    /// <p>
    /// Characters are represented as numeric values according to the Unicode standard.
    /// The numeric values are represented with 8-bit code units.
    /// A single character can be represented with 1, 2, 3, or 4 code units.
    /// </p>
    Utf8,
    /// Unicode UTF-16 encoding.
    /// <p>
    /// Characters are represented as numeric values according to the Unicode standard.
    /// The numeric values are represented with 16-bit code units.
    /// A single character can be represented with 1 or 2 code units.
    /// (A pair of code units representing a single character is called a surrogate pair.)
    /// </p>
    Utf16,
    /// Unicode UTF-32 encoding.
    /// <p>
    /// Characters are represented as numeric values according to the Unicode standard.
    /// The numeric values are represented with 32-bit code units.
    /// A complete character is always represented with a single code unit.
    /// </p>
    Utf32,
}

impl Display for StringEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Byte => "Byte".to_string(),
                Self::Utf8 => "Utf8".to_string(),
                Self::Utf16 => "Utf16".to_string(),
                Self::Utf32 => "Utf32".to_string(),
            }
        )
    }
}

/// StringStorage defines an enumeration that captures the supported storage
/// characteristics for strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringStorage {
    /// The string is stored as a fixed-size string.
    /// StringBuilder's builder will panic if the string's encoding is not Byte or UTF-32.
    FixedSize(u64),
    /// The string is stored as a fixed-capacity string, where size can grow upto but not beyond the specified capacity.
    FixedCapacity(u64),
    /// The string is stored as a string with a specified initial capacity.
    /// This is useful for performance optimization when a reasonable mimimim size of the string is known in advance.
    InitialCapacity(u64),
    /// The string is stored as a variable-size string.
    VariableSize,
}

impl Display for StringStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::FixedSize(size) => format!("FixedSize({})", size),
                Self::FixedCapacity(capacity) => format!("FixedCapacity({})", capacity),
                Self::InitialCapacity(capacity) => format!("InitialCapacity({})", capacity),
                Self::VariableSize => "VariableSize".to_string(),
            }
        )
    }
}

/// A primitive spec for strings.
#[derive(Debug, PartialEq)]
pub struct StringSpec {
    encoding: StringEncoding,
    storage: Option<StringStorage>,
}

impl StringSpec {
    /// Returns an initialized string spec.
    pub(crate) fn new(encoding: StringEncoding, storage: Option<StringStorage>) -> StringSpec {
        StringSpec {
            encoding: (encoding),
            storage: (storage),
        }
    }

    /// Returns the string's encoding.
    pub fn encoding(&self) -> &StringEncoding {
        &self.encoding
    }

    /// Returns the string's storage.
    pub fn storage(&self) -> &Option<StringStorage> {
        &self.storage
    }
}

impl SpecCompatibility for StringSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        if self.encoding != required.encoding {
            return false;
        }
        if let Some(storage) = &self.storage {
            if let Some(required_storage) = &required.storage {
                return storage == required_storage;
            }
        } else if required.storage.is_some() {
            return false;
        }
        true
    }
}

impl IsOrdered for StringSpec {
    fn is_ordered(&self) -> bool {
        true // Strings are ordered.
    }
}

impl PrimitiveSpec for StringSpec {}

impl Display for StringSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "String {{ encoding: {}, storage: {} }}",
            self.encoding,
            self.storage
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
