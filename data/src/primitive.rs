use std::fmt::{Debug, Display};

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
use crate::accessors::temporal::day_second_interval::DaySecondInterval;
use crate::accessors::tuple::Tuple;
use crate::primitive_category::PrimitiveCategory;
use crate::primitive_def::{IsOrdered, PrimitiveDef};
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
use crate::accessors::enum_object::EnumObject;
use crate::accessors::float::Float;
use crate::accessors::integer::Integer;
use crate::accessors::strings::byte_string::ByteString;
use crate::accessors::strings::utf8_string::Utf8String;
use crate::accessors::strings::utf16_string::Utf16String;
use crate::accessors::strings::utf32_string::Utf32String;
use crate::accessors::temporal::date::Date;
use crate::accessors::temporal::local_date_time::LocalDateTime;
use crate::accessors::temporal::local_time::LocalTime;
use crate::accessors::temporal::year_month_interval::YearMonthInterval;
use crate::accessors::temporal::zoned_date_time::ZonedDateTime;
use crate::accessors::temporal::zoned_time::ZonedTime;
use crate::primitive_specs::tuple_spec::TupleSpec;
use crate::primitive_specs::walk_spec::WalkSpec;
use crate::spec_compatibility::SpecCompatibility;

// TODO: Add a decimal primitive.

/// Primitive captures the different types of supported primitives.
pub enum Primitive {
    /// A value that can either be 'true' or 'false'.
    Boolean(Option<PrimitiveDef<BooleanSpec, Boolean>>),
    /// A character alone or within a string.
    Character(Option<PrimitiveDef<CharacterSpec, Character>>),
    /// A signed or unsigned whole number.src/primitive.rs
    Integer(Option<PrimitiveDef<IntegerSpec, Integer>>),
    /// A number with a fractional part.
    Float(Option<PrimitiveDef<FloatSpec, Float>>),
    /// A string of bytes(e.g. ASCII).
    ByteString(Option<PrimitiveDef<StringSpec, ByteString>>),
    /// A string of UTF-8 code units.
    Utf8String(Option<PrimitiveDef<StringSpec, Utf8String>>),
    /// A string of UTF-16 code units.
    Utf16String(Option<PrimitiveDef<StringSpec, Utf16String>>),
    /// A string of UTF-32 code units.
    Utf32String(Option<PrimitiveDef<StringSpec, Utf32String>>),
    /// A date on a calendar.
    Date(Option<PrimitiveDef<DateSpec, Date>>),
    /// A local time of day.
    LocalTime(Option<PrimitiveDef<TimeSpec, LocalTime>>),
    /// A zoned time of day.
    ZonedTime(Option<PrimitiveDef<TimeSpec, ZonedTime>>),
    /// A local date-and-time value.
    LocalDateTime(Option<PrimitiveDef<DateTimeSpec, LocalDateTime>>),
    /// A zoned date-and-time value.
    ZonedDateTime(Option<PrimitiveDef<DateTimeSpec, ZonedDateTime>>),
    /// A difference between two dates, times, or date-and-time values expressed as a year-month interval.
    YearMonthInterval(Option<PrimitiveDef<IntervalSpec, YearMonthInterval>>),
    /// A difference between two dates, times, or date-and-time values expressed as a day-second interval.
    DaySecondInterval(Option<PrimitiveDef<IntervalSpec, DaySecondInterval>>),
    /// An enumeration value.
    EnumObject(Option<PrimitiveDef<EnumObjectSpec, EnumObject>>),
    /// A Data Specification value.
    DataSpec(Option<PrimitiveDef<DataSpecSpec, DataSpec>>),
    /// A globally unique identifier (GUID), also called a universally unique
    /// identifier (UUID).
    Guid(Option<PrimitiveDef<GuidSpec, Guid>>),
    /// A reference to an object of a class.
    Reference(Option<PrimitiveDef<ReferenceSpec, Reference>>),
    /// An object of a class.
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
    /// A finite ordered list of values.
    Tuple(Option<PrimitiveDef<TupleSpec, Tuple>>),
    /// A Binary Large Object value.
    Blob(Option<PrimitiveDef<BlobSpec, Blob>>),
    /// An object identifier
    Identifier(Option<PrimitiveDef<IdentifierSpec, Identifier>>),
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
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Character(p), Self::Character(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Integer(p), Self::Integer(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Float(p), Self::Float(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::EnumObject(p), Self::EnumObject(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Date(p), Self::Date(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::LocalTime(p), Self::LocalTime(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::ZonedTime(p), Self::ZonedTime(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::LocalDateTime(p), Self::LocalDateTime(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::ZonedDateTime(p), Self::ZonedDateTime(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::ByteString(p), Self::ByteString(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Utf8String(p), Self::Utf8String(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Utf16String(p), Self::Utf16String(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Utf32String(p), Self::Utf32String(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::DataSpec(p), Self::DataSpec(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::YearMonthInterval(p), Self::YearMonthInterval(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::DaySecondInterval(p), Self::DaySecondInterval(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Guid(p), Self::Guid(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Reference(p), Self::Reference(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Object(p), Self::Object(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::List(p), Self::List(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Set(p), Self::Set(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Map(p), Self::Map(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Sequence(p), Self::Sequence(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Edge(p), Self::Edge(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Walk(p), Self::Walk(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::EnumClass(p), Self::EnumClass(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Property(p), Self::Property(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Attribute(p), Self::Attribute(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Class(p), Self::Class(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Tuple(p), Self::Tuple(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Blob(p), Self::Blob(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
                }
            }
            (Self::Identifier(p), Self::Identifier(r)) => {
                if p.is_some() && r.is_some() {
                    p.as_ref()
                        .unwrap()
                        .spec()
                        .is_compatible_with(r.as_ref().unwrap().spec())
                } else {
                    !(p.is_none() && r.is_some())
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
            PrimitiveCategory::Interval => self.is_interval(),
            PrimitiveCategory::DateTime => self.is_date_time(),
            PrimitiveCategory::Time => self.is_time(),
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

    /// Returns true if the primitive belongs to the date-time category; false otherwise.
    pub fn is_date_time(&self) -> bool {
        matches!(*self, Self::LocalDateTime(..) | Self::ZonedDateTime(..))
    }

    /// Returns true if the primitive belongs to the time category; false otherwise.
    pub fn is_time(&self) -> bool {
        matches!(*self, Self::LocalTime(..) | Self::ZonedTime(..))
    }

    /// Returns true if the primitive belongs to the interval category; false otherwise.
    pub fn is_interval(&self) -> bool {
        matches!(
            *self,
            Self::YearMonthInterval(..) | Self::DaySecondInterval(..)
        )
    }

    /// Returns true if the primitive belongs to the basic category; false otherwise.
    pub fn is_basic(&self) -> bool {
        if self.is_numberic() || self.is_date_time() || self.is_time() || self.is_interval() {
            return true;
        };
        matches!(
            *self,
            Self::Boolean(..)
                | Self::Character(..)
                | Self::EnumObject(..)
                | Self::Date(..)
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

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Boolean".to_string()
                    }
                }
                Self::Character(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Character".to_string()
                    }
                }
                Self::Integer(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Integer".to_string()
                    }
                }
                Self::Float(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Float".to_string()
                    }
                }
                Self::EnumObject(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "EnumObject".to_string()
                    }
                }
                Self::Date(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Date".to_string()
                    }
                }
                Self::LocalTime(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "LocalTime".to_string()
                    }
                }
                Self::ZonedTime(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "ZonedTime".to_string()
                    }
                }
                Self::LocalDateTime(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "LocalDateTime".to_string()
                    }
                }
                Self::ZonedDateTime(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "ZonedDateTime".to_string()
                    }
                }
                Self::ByteString(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "ByteString".to_string()
                    }
                }
                Self::Utf8String(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Utf8String".to_string()
                    }
                }
                Self::Utf16String(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Utf16String".to_string()
                    }
                }
                Self::Utf32String(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Utf32String".to_string()
                    }
                }
                Self::DataSpec(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "DataSpec".to_string()
                    }
                }
                Self::YearMonthInterval(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "YearMonthInterval".to_string()
                    }
                }
                Self::DaySecondInterval(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "DaySecondInterval".to_string()
                    }
                }
                Self::Guid(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Guid".to_string()
                    }
                }
                Self::Reference(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Reference".to_string()
                    }
                }
                Self::Object(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Object".to_string()
                    }
                }
                Self::List(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "List".to_string()
                    }
                }
                Self::Set(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Set".to_string()
                    }
                }
                Self::Map(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Map".to_string()
                    }
                }
                Self::Sequence(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Sequence".to_string()
                    }
                }
                Self::Edge(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Edge".to_string()
                    }
                }
                Self::Walk(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Walk".to_string()
                    }
                }
                Self::EnumClass(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "EnumClass".to_string()
                    }
                }
                Self::Property(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Property".to_string()
                    }
                }
                Self::Attribute(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Attribute".to_string()
                    }
                }
                Self::Class(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Class".to_string()
                    }
                }
                Self::Tuple(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Tuple".to_string()
                    }
                }
                Self::Blob(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Blob".to_string()
                    }
                }
                Self::Identifier(def) => {
                    if let Some(as_def) = def {
                        as_def.spec().to_string()
                    } else {
                        "Identifier".to_string()
                    }
                }
            }
        )
    }
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Primitive({})", self)
    }
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(p), Self::Boolean(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Character(p), Self::Character(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Integer(p), Self::Integer(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Float(p), Self::Float(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::EnumObject(p), Self::EnumObject(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Date(p), Self::Date(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::LocalTime(p), Self::LocalTime(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::ZonedTime(p), Self::ZonedTime(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::LocalDateTime(p), Self::LocalDateTime(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::ZonedDateTime(p), Self::ZonedDateTime(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::ByteString(p), Self::ByteString(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Utf8String(p), Self::Utf8String(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Utf16String(p), Self::Utf16String(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Utf32String(p), Self::Utf32String(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::DataSpec(p), Self::DataSpec(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::YearMonthInterval(p), Self::YearMonthInterval(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Guid(p), Self::Guid(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Reference(p), Self::Reference(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Object(p), Self::Object(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::List(p), Self::List(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Set(p), Self::Set(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Map(p), Self::Map(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Sequence(p), Self::Sequence(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Edge(p), Self::Edge(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Walk(p), Self::Walk(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::EnumClass(p), Self::EnumClass(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Property(p), Self::Property(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Attribute(p), Self::Attribute(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Class(p), Self::Class(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Tuple(p), Self::Tuple(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Blob(p), Self::Blob(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            (Self::Identifier(p), Self::Identifier(r)) => {
                if let (Some(p), Some(r)) = (p, r) {
                    p.spec() == r.spec()
                } else {
                    p.is_none() && r.is_none()
                }
            }
            // If the primitives are of different types, they are not equal.
            _ => false,
        }
    }
}

impl IsOrdered for Primitive {
    fn is_ordered(&self) -> bool {
        match self {
            Self::Boolean(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Character(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Integer(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Float(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::EnumObject(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Date(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::LocalTime(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::ZonedTime(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::LocalDateTime(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::ZonedDateTime(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::ByteString(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Utf8String(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Utf16String(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Utf32String(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::DataSpec(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::YearMonthInterval(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::DaySecondInterval(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Guid(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Reference(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Object(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::List(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Set(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Map(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Sequence(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Edge(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Walk(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::EnumClass(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Property(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Attribute(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Class(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Tuple(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Blob(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
            Self::Identifier(p) => p.as_ref().is_none_or(|p| p.spec().is_ordered()),
        }
    }
}
