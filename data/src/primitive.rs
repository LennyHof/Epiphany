use std::fmt::Display;

use crate::accessors::collections::list::List;
use crate::accessors::collections::map::Map;
use crate::accessors::collections::set::Set;
use crate::primitive_def::PrimitiveDef;
use crate::primitive_specs::boolean_spec::BooleanSpec;
use crate::primitive_specs::character_spec::CharacterSpec;
use crate::primitive_specs::data_spec_spec::DataSpecSpec;
use crate::primitive_specs::date_spec::DateSpec;
use crate::primitive_specs::date_time_spec::DateTimeSpec;
use crate::primitive_specs::enum_object_spec::EnumObjectSpec;
use crate::primitive_specs::float_spec::FloatSpec;
use crate::primitive_specs::integer_spec::IntegerSpec;
use crate::primitive_specs::interval_spec::IntervalSpec;
use crate::primitive_specs::list_spec::ListSpec;
use crate::primitive_specs::map_spec::MapSpec;
use crate::primitive_specs::set_spec::SetSpec;
use crate::primitive_specs::string_spec::StringSpec;
use crate::primitive_specs::time_spec::TimeSpec;

use crate::accessors::boolean::Boolean;
use crate::accessors::character::Character;
use crate::accessors::data_spec::DataSpec;
use crate::accessors::date_times::date::Date;
use crate::accessors::date_times::date_time::DateTime;
use crate::accessors::date_times::interval::Interval;
use crate::accessors::date_times::time::Time;
use crate::accessors::enum_object::EnumObject;
use crate::accessors::float::Float;
use crate::accessors::integer::Integer;
use crate::accessors::strings::byte_string::ByteString;
use crate::accessors::strings::utf8_string::Utf8String;
use crate::accessors::strings::utf16_string::Utf16String;
use crate::accessors::strings::utf32_string::Utf32String;

// TODO: Add a decimal primitive.

/// Primitive captures the different types of supported primitives.
//#[derive(Copy, Clone)]
pub enum Primitive {
    /// A value that can either be 'true' or 'false'.
    Boolean(Option<PrimitiveDef<BooleanSpec, Boolean>>),
    /// A character alone or within a string.
    Character(Option<PrimitiveDef<CharacterSpec, Character>>),
    /// A signed or unsigned whole number.src/primitive.rs
    Integer(Option<PrimitiveDef<IntegerSpec, Integer>>),
    /// A number with a fractional part.
    Float(Option<PrimitiveDef<FloatSpec, Float>>),
    /// An enumeration value.
    EnumObject(Option<PrimitiveDef<EnumObjectSpec, EnumObject>>),
    /// A date on a calendar.
    Date(Option<PrimitiveDef<DateSpec, Date>>),
    /// A time of day.
    Time(Option<PrimitiveDef<TimeSpec, Time>>),
    /// A date-and-time value.
    DateTime(Option<PrimitiveDef<DateTimeSpec, DateTime>>),
    /// A string of bytes(e.g. ASCII).
    ByteString(Option<PrimitiveDef<StringSpec, ByteString>>),
    /// A string of UTF-8 code units.
    Utf8String(Option<PrimitiveDef<StringSpec, Utf8String>>),
    /// A string of UTF-16 code units.
    Utf16String(Option<PrimitiveDef<StringSpec, Utf16String>>),
    /// A string of UTF-32 code units.
    Utf32String(Option<PrimitiveDef<StringSpec, Utf32String>>),
    /// A Data Specification value.
    DataSpec(Option<PrimitiveDef<DataSpecSpec, DataSpec>>),
    /// A difference between two dates, times, or date-and-time values.
    Interval(Option<PrimitiveDef<IntervalSpec, Interval>>),
    /// A globally unique identifier (GUID), also called a universally unique
    /// identifier (UUID).
    Guid,
    /// A reference to an object of a schema class; an object identifier (OID).
    Reference,
    /// An object of a schema class.
    Object,
    /// An ordered grouping of potentially non-unique values.
    List(Option<PrimitiveDef<ListSpec, List>>),
    /// An ordered or unordered grouping of unique values.
    Set(Option<PrimitiveDef<SetSpec, Set>>),
    /// A ordered or unordered grouping of unique values representing keys that lead
    /// to other potentially non-unique values.
    Map(Option<PrimitiveDef<MapSpec, Map>>),
    /// A sequence that can return 0 or more values in succession; supports
    /// enumeration (iteration).
    Sequence,
    /// A component of a navigational walk between two objects (vertices) in a
    /// graph.
    Edge,
    /// A component in a graph.
    Walk,
    /// An enumeration class
    EnumClass,
    /// A specification of property values that may be added to individual objects.
    Property,
    /// A specification of attribute values that belong to every object of a class.
    Attribute,
    /// A class of objects.
    Class,
    /// A finite ordered list of elements.
    Tuple,
    /// A Binary Large Object value.
    Blob,
    /// An object identifier
    Identifier,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Boolean(_) => "Boolean".to_string(),
                Self::Character(_) => "Character".to_string(),
                Self::Integer(_) => "Integer".to_string(),
                Self::Float(_) => "Float".to_string(),
                Self::EnumObject(_) => "EnumObject".to_string(),
                Self::Date(_) => "Date".to_string(),
                Self::Time(_) => "Time".to_string(),
                Self::DateTime(_) => "DateTime".to_string(),
                Self::ByteString(_) => "ByteString".to_string(),
                Self::Utf8String(_) => "Utf8String".to_string(),
                Self::Utf16String(_) => "Utf16String".to_string(),
                Self::Utf32String(_) => "Utf32String".to_string(),
                Self::DataSpec(_) => "DataSpec".to_string(),
                Self::Interval(_) => "Interval".to_string(),
                Self::Guid => "Guid".to_string(),
                Self::Reference => "Reference".to_string(),
                Self::Object => "Object".to_string(),
                Self::List(_) => "List".to_string(),
                Self::Set(_) => "Set".to_string(),
                Self::Map(_) => "Map".to_string(),
                Self::Sequence => "Sequence".to_string(),
                Self::Edge => "Edge".to_string(),
                Self::Walk => "Walk".to_string(),
                Self::EnumClass => "EnumClass".to_string(),
                Self::Property => "Property".to_string(),
                Self::Attribute => "Attribute".to_string(),
                Self::Class => "Class".to_string(),
                Self::Tuple => "Tuple".to_string(),
                Self::Blob => "Blob".to_string(),
                Self::Identifier => "Identifier".to_string(),
            }
        )
    }
}

impl Primitive {
    /// Returns true if the primitive belongs to the numeric category; false otherwise.
    pub fn is_numberic(&self) -> bool {
        matches!(*self, Self::Integer(..) | Self::Float(..))
    }

    /// Returns true if the primitive belongs to the basic category; false otherwise.
    pub fn is_basic(&self) -> bool {
        if self.is_numberic() {
            return true;
        };
        matches!(
            *self,
            Self::Boolean(..)
                | Self::Character(..)
                | Self::EnumObject(..)
                | Self::Date(..)
                | Self::Time(..)
                | Self::DateTime(..)
                | Self::Interval(..)
                | Self::Guid
                | Self::Identifier
        )
    }

    /// Returns true if the primitive belongs to the string category; false otherwise.
    pub fn is_string(&self) -> bool {
        matches!(
            *self,
            Self::ByteString(..)
                | Self::Utf8String(..)
                | Self::Utf16String(..)
                | Self::Utf32String(..)
        )
    }

    /// Returns true if the primitive belongs to the collection category; false otherwise.
    pub fn is_collection(&self) -> bool {
        matches!(*self, Self::List(_) | Self::Set(_) | Self::Map(_))
    }

    /// Returns true if the primitive belongs to the object-or-reference category; false otherwise.
    pub fn is_object_or_reference(&self) -> bool {
        matches!(*self, Self::Object | Self::Reference)
    }

    /// Returns true if the primitive belongs to the is-sequenceable category; false otherwise.
    pub fn is_sequenceable(&self) -> bool {
        matches!(
            *self,
            Self::List(_) | Self::Set(_) | Self::Map(_) | Self::Sequence
        )
    }

    /// Returns true if the primitive belongs to the is-schema category; false otherwise.
    pub fn is_schema(&self) -> bool {
        matches!(
            *self,
            Self::Class | Self::EnumClass | Self::Attribute | Self::Property
        )
    }

    /// Returns true if the primitive belongs to the all category; false otherwise.
    pub fn is_all(&self) -> bool {
        true
    }
}
