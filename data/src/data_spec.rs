use std::fmt::Display;

use super::primitive::Primitive;
use super::primitive_category::PrimitiveCategory;

/// Specifies if the data specification is for compare only or is valid for data access.
#[derive(Copy, Clone, PartialEq)]
pub enum DataSpecLevel {
    /// The data specification is valid only for comparison purposes.
    Compare,
    /// The data specification is valid for access and comparison purposes.
    Access,
}

impl Display for DataSpecLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DataSpecLevel::Compare => "Compare".to_string(),
                DataSpecLevel::Access => "Access".to_string(),
            }
        )
    }
}

/// Specifies the data specification's type.
pub enum DataSpecType {
    /// No type.
    None,
    /// Logical type.
    Primitive(Primitive),
    /// Logical-type category
    PrimitiveCategory(PrimitiveCategory),
}

///
/// A DataSpec expresses constraints on values.
///
///<p>
/// Data specifications serve several purposes: </p>
/// <ul>
/// <li> Constrain the possible values a variable can hold. </li>
///
/// <li> Provide sufficient runtime information about values to enable the application to
/// interpret and use them.
///</li>
/// <li> Provide sufficient details about values received by a query operator,
/// so the operator can determine whether they are valid operands. </li>
/// </ul>
///
/// <p>
/// At a minimum, a data specification specifies a logical type or logical-type category,
/// and may, but need not, include other characteristics, such as encoding and storage options.
/// </p>
/// <p>
/// An application uses a DataSpecBuilder to build a data specification.
/// </p>
///
pub struct DataSpec {
    specification_level: DataSpecLevel,
    specification_type: DataSpecType,
}

impl DataSpec {
    /// Returns a DataSpec initialized to represent a primitive.
    pub fn new_primitive(primitive: Primitive, specification_level: DataSpecLevel) -> DataSpec {
        DataSpec {
            specification_type: DataSpecType::Primitive(primitive),
            specification_level,
        }
    }

    /// Returns the data specification's specification level
    pub fn specification_level(&self) -> DataSpecLevel {
        self.specification_level
    }

    /// Returns the specification type.
    pub fn specification_type(&self) -> &DataSpecType {
        &self.specification_type
    }

    /// Returns the specification type as a mutable reference.
    pub fn specification_type_mut(&mut self) -> &mut DataSpecType {
        &mut self.specification_type
    }
}
