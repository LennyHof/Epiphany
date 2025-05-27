use std::fmt::Display;

// TODO: Add a Real category.

/// PrimitiveCategory captures the different categories that Primitives may belong.
#[derive(Copy, Clone)]
pub enum PrimitiveCategory {
    /// Category of primitives representing numbers.
    Numeric,
    /// Category of primitives that are sequences of characters.
    String,
    /// Category of primitives representing basic data.
    Basic,
    /// Category of primitives representing basic data or strings.
    Simple,
    /// Category of primitives representing objects or references to objects.
    ObjectOrReference,
    /// Category of primitives representing collections.
    Collection,
    /// Category of primitives representing collections or sequences.
    Sequenceable,
    /// Category of primitives representing entities in a schema.
    Schema,
    /// Category that includes all supported primitives.
    All,
}

impl Display for PrimitiveCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Numeric => "Numeric".to_string(),
                Self::String => "String".to_string(),
                Self::Basic => "Basic".to_string(),
                Self::Simple => "Simple".to_string(),
                Self::ObjectOrReference => "ObjectOrReference".to_string(),
                Self::Collection => "Collection".to_string(),
                Self::Sequenceable => "Sequenceable".to_string(),
                Self::Schema => "Schema".to_string(),
                Self::All => "All".to_string(),
            }
        )
    }
}
