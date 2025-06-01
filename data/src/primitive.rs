use std::fmt::Display;

use crate::accessors::blob::Blob;
use crate::accessors::collections::list::List;
use crate::accessors::collections::map::Map;
use crate::accessors::collections::set::Set;
use crate::accessors::graph::edge::Edge;
use crate::accessors::graph::walk::Walk;
use crate::accessors::guid::Guid;
use crate::accessors::identifier::Identifier;
use crate::accessors::object::Object;
use crate::accessors::reference::Reference;
use crate::accessors::schema::attribute::Attribute;
use crate::accessors::schema::class::Class;
use crate::accessors::schema::enum_class::EnumClass;
use crate::accessors::schema::property::Property;
use crate::accessors::sequence::Sequence;
use crate::accessors::tuple::Tuple;
use crate::primitive_category::PrimitiveCategory;
use crate::primitive_def::PrimitiveDef;
use crate::primitive_specs::attribute_spec::AttributeSpec;
use crate::primitive_specs::blob_spec::BlobSpec;
use crate::primitive_specs::boolean_spec::BooleanSpec;
use crate::primitive_specs::character_spec::CharacterSpec;
use crate::primitive_specs::class_spec::ClassSpec;
use crate::primitive_specs::data_spec_spec::DataSpecSpec;
use crate::primitive_specs::date_spec::DateSpec;
use crate::primitive_specs::date_time_spec::DateTimeSpec;
use crate::primitive_specs::edge_spec::EdgeSpec;
use crate::primitive_specs::enum_class_spec::EnumClassSpec;
use crate::primitive_specs::enum_object_spec::EnumObjectSpec;
use crate::primitive_specs::float_spec::FloatSpec;
use crate::primitive_specs::guid_spec::GuidSpec;
use crate::primitive_specs::identifier_spec::IdentifierSpec;
use crate::primitive_specs::integer_spec::IntegerSpec;
use crate::primitive_specs::interval_spec::IntervalSpec;
use crate::primitive_specs::list_spec::ListSpec;
use crate::primitive_specs::map_spec::MapSpec;
use crate::primitive_specs::object_spec::ObjectSpec;
use crate::primitive_specs::property_spec::PropertySpec;
use crate::primitive_specs::reference_spec::ReferenceSpec;
use crate::primitive_specs::sequence_spec::SequenceSpec;
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
use crate::primitive_specs::tuple_spec::TupleSpec;
use crate::primitive_specs::walk_spec::WalkSpec;

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
    Guid(Option<PrimitiveDef<GuidSpec, Guid>>),
    /// A reference to an object of a schema class; an object identifier (OID).
    Reference(Option<PrimitiveDef<ReferenceSpec, Reference>>),
    /// An object of a schema class.
    Object(Option<PrimitiveDef<ObjectSpec, Object>>),
    /// An ordered grouping of potentially non-unique values.
    List(Option<PrimitiveDef<ListSpec, List>>),
    /// An ordered or unordered grouping of unique values.
    Set(Option<PrimitiveDef<SetSpec, Set>>),
    /// A ordered or unordered grouping of unique values representing keys that lead
    /// to other potentially non-unique values.
    Map(Option<PrimitiveDef<MapSpec, Map>>),
    /// A sequence that can return 0 or more values in succession; supports
    /// enumeration (iteration).
    Sequence(Option<PrimitiveDef<SequenceSpec, Sequence>>),
    /// A component of a navigational walk between two objects (vertices) in a
    /// graph.
    Edge(Option<PrimitiveDef<EdgeSpec, Edge>>),
    /// A component in a graph.
    Walk(Option<PrimitiveDef<WalkSpec, Walk>>),
    /// An enumeration class
    EnumClass(Option<PrimitiveDef<EnumClassSpec, EnumClass>>),
    /// A specification of property values that may be added to individual objects.
    Property(Option<PrimitiveDef<PropertySpec, Property>>),
    /// A specification of attribute values that belong to every object of a class.
    Attribute(Option<PrimitiveDef<AttributeSpec, Attribute>>),
    /// A class of objects.
    Class(Option<PrimitiveDef<ClassSpec, Class>>),
    /// A finite ordered list of elements.
    Tuple(Option<PrimitiveDef<TupleSpec, Tuple>>),
    /// A Binary Large Object value.
    Blob(Option<PrimitiveDef<BlobSpec, Blob>>),
    /// An object identifier
    Identifier(Option<PrimitiveDef<IdentifierSpec, Identifier>>),
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
                Self::Guid(_) => "Guid".to_string(),
                Self::Reference(_) => "Reference".to_string(),
                Self::Object(_) => "Object".to_string(),
                Self::List(_) => "List".to_string(),
                Self::Set(_) => "Set".to_string(),
                Self::Map(_) => "Map".to_string(),
                Self::Sequence(_) => "Sequence".to_string(),
                Self::Edge(_) => "Edge".to_string(),
                Self::Walk(_) => "Walk".to_string(),
                Self::EnumClass(_) => "EnumClass".to_string(),
                Self::Property(_) => "Property".to_string(),
                Self::Attribute(_) => "Attribute".to_string(),
                Self::Class(_) => "Class".to_string(),
                Self::Tuple(_) => "Tuple".to_string(),
                Self::Blob(_) => "Blob".to_string(),
                Self::Identifier(_) => "Identifier".to_string(),
            }
        )
    }
}

impl Primitive {
    /// Returns if the primitive is compatible with a required primitive.
    pub fn is_compatible_with(&self, required: &Primitive) -> bool {
        match (self, required) {
            (Self::Boolean(p), Self::Boolean(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Character(p), Self::Character(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Integer(p), Self::Integer(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Float(p), Self::Float(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::EnumObject(p), Self::EnumObject(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Date(p), Self::Date(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Time(p), Self::Time(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::DateTime(p), Self::DateTime(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::ByteString(p), Self::ByteString(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Utf8String(p), Self::Utf8String(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Utf16String(p), Self::Utf16String(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Utf32String(p), Self::Utf32String(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::DataSpec(p), Self::DataSpec(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Interval(p), Self::Interval(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Guid(p), Self::Guid(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Reference(p), Self::Reference(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Object(p), Self::Object(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::List(p), Self::List(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Set(p), Self::Set(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Map(p), Self::Map(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Sequence(p), Self::Sequence(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Edge(p), Self::Edge(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Walk(p), Self::Walk(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::EnumClass(p), Self::EnumClass(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Property(p), Self::Property(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Attribute(p), Self::Attribute(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Class(p), Self::Class(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Tuple(p), Self::Tuple(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Blob(p), Self::Blob(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            (Self::Identifier(p), Self::Identifier(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else if p.is_none() && r.is_some() {
                    false
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    /// Returns true if the primitive is compatible with the required category; false otherwise.
    pub fn is_compatible_with_category(&self, required: &PrimitiveCategory) -> bool {
        match *required {
            PrimitiveCategory::Numeric => self.is_numberic(),
            PrimitiveCategory::String => self.is_string(),
            PrimitiveCategory::Basic => self.is_basic(),
            PrimitiveCategory::Simple => self.is_basic() || self.is_string(),
            PrimitiveCategory::ObjectOrReference => self.is_object_or_reference(),
            PrimitiveCategory::Collection => self.is_collection(),
            PrimitiveCategory::Sequenceable => self.is_sequenceable(),
            PrimitiveCategory::Schema => self.is_schema(),
            PrimitiveCategory::All => true,
        }
    }

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
                | Self::Guid(..)
                | Self::Identifier(..)
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
        matches!(*self, Self::Object(_) | Self::Reference(_))
    }

    /// Returns true if the primitive belongs to the is-sequenceable category; false otherwise.
    pub fn is_sequenceable(&self) -> bool {
        matches!(
            *self,
            Self::List(_) | Self::Set(_) | Self::Map(_) | Self::Sequence(_)
        )
    }

    /// Returns true if the primitive belongs to the is-schema category; false otherwise.
    pub fn is_schema(&self) -> bool {
        matches!(
            *self,
            Self::Class(_) | Self::EnumClass(_) | Self::Attribute(_) | Self::Property(_)
        )
    }

    /// Returns true if the primitive belongs to the all category; false otherwise.
    pub fn is_all(&self) -> bool {
        true
    }
}
