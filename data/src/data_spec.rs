use std::fmt::Display;

use crate::spec_compatibility::SpecCompatibility;

use super::primitive::Primitive;
use super::primitive_category::PrimitiveCategory;

/// Specifies if the data specification is for compare only or is valid for data access.
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

    /// Returns a DataSpec initialized to represent a primitive category.
    pub fn new_primitive_category(primitive_category: PrimitiveCategory) -> DataSpec {
        DataSpec {
            specification_type: DataSpecType::PrimitiveCategory(primitive_category),
            specification_level: DataSpecLevel::Compare,
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

impl Default for DataSpec {
    fn default() -> Self {
        DataSpec {
            specification_level: DataSpecLevel::Compare,
            specification_type: DataSpecType::None,
        }
    }
}

impl Display for DataSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.specification_type {
                DataSpecType::None => "None".to_string(),
                DataSpecType::Primitive(p) => p.to_string(),
                DataSpecType::PrimitiveCategory(c) => c.to_string(),
            }
        )
    }
}

impl SpecCompatibility for DataSpec {
    /// Checks if this spec is compatible with the required spec.
    fn is_compatible_with(&self, required: &Self) -> bool {
        // Check if the specification levels are compatible
        if required.specification_level == DataSpecLevel::Access
            && self.specification_level == DataSpecLevel::Compare
        {
            return false;
        }

        // Check if the specification types are compatible
        match (&self.specification_type, &required.specification_type) {
            (DataSpecType::None, DataSpecType::None) => true,
            (DataSpecType::Primitive(p1), DataSpecType::Primitive(p2)) => p1.is_compatible_with(p2),
            (DataSpecType::PrimitiveCategory(c1), DataSpecType::PrimitiveCategory(c2)) => {
                c1.is_compatible_with(c2)
            }
            (DataSpecType::Primitive(p), DataSpecType::PrimitiveCategory(c)) => {
                p.is_compatible_with_category(c)
            }
            _ => false,
        }
    }
}
