use std::{fmt::Display, hash::Hash};

use crate::{
    accessors::{
        boolean::Boolean,
        collections::{list::List, map::Map, set::Set},
        float::Float,
        integer::Integer,
    },
    data_provider::{DataProvider, default_data_provider},
    data_spec::{DataSpec, DataSpecLevel, DataSpecType},
    primitive::Primitive,
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecCompatibility,
};

/// A Variable holds, and provides access to primitives.
pub struct Variable {
    data_spec: DataSpec,
}

impl Variable {
    /// Creates a new Variable based on the provided DataSpec.
    /// Uses the default data provider to initialize the variable.
    pub fn new(data_spec: &DataSpec) -> Variable {
        default_data_provider().variable_for(data_spec)
    }

    /// Returns an initialized Variable based on a given primitive.
    pub fn new_primitive(primitive: Primitive) -> Variable {
        Variable {
            data_spec: (DataSpec::new_primitive(primitive, DataSpecLevel::Access)),
        }
    }

    /// Returns the variable's data specification.
    pub fn data_spec(&self) -> &DataSpec {
        &self.data_spec
    }

    /// Extracts and returns the Integer accessor within the variable.
    /// Panics if unable to do so.
    pub fn integer(&self) -> &Integer {
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Integer(integer_def) => {
                    let def = &integer_def.as_ref().unwrap();
                    def.borrow_access()
                }
                _ => panic!("Not an integer."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the Integer accessor within the variable as mutable.
    /// Panics if unable to do so.
    pub fn integer_mut(&mut self) -> &mut Integer {
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Integer(integer_def) => {
                    let def = integer_def.as_mut().unwrap();
                    def.mut_access()
                }
                _ => panic!("Not an integer."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the Float accessor within the variable.
    /// Panics if unable to do so.
    pub fn float(&self) -> &Float {
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Float(float_def) => {
                    let def = &float_def.as_ref().unwrap();
                    def.borrow_access()
                }
                _ => panic!("Not a float."),
            },
            _ => panic!("Not a primitive."),
        }
    }
    /// Extracts and returns the Float accessor within the variable as mutable.
    /// Panics if unable to do so.
    pub fn float_mut(&mut self) -> &mut Float {
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Float(float_def) => {
                    let def = float_def.as_mut().unwrap();
                    def.mut_access()
                }
                _ => panic!("Not a float."),
            },
            _ => panic!("Not a primitive."),
        }
    }
    /// Extracts and returns the Boolean accessor within the variable.
    /// Panics if unable to do so.
    pub fn boolean(&self) -> &Boolean {
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Boolean(boolean_def) => {
                    let def = &boolean_def.as_ref().unwrap();
                    def.borrow_access()
                }
                _ => panic!("Not a boolean."),
            },
            _ => panic!("Not a primitive."),
        }
    }
    /// Extracts and returns the Boolean accessor within the variable as mutable.
    /// Panics if unable to do so.
    pub fn boolean_mut(&mut self) -> &mut Boolean {
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Boolean(boolean_def) => {
                    let def = boolean_def.as_mut().unwrap();
                    def.mut_access()
                }
                _ => panic!("Not a boolean."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the List accessor within the variable.
    /// Panics if unable to do so.
    pub fn list(&self) -> &List {
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::List(list_def) => {
                    let def = &list_def.as_ref().unwrap();
                    def.borrow_access()
                }
                _ => panic!("Not a list."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the List accessor within the variable as mutable.
    /// Panics if unable to do so.
    pub fn list_mut(&mut self) -> &mut List {
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::List(list_def) => {
                    let def = list_def.as_mut().unwrap();
                    def.mut_access()
                }
                _ => panic!("Not a list."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the Set accessor within the variable.
    /// Panics if unable to do so.
    pub fn set(&self) -> &Set {
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Set(set_def) => {
                    let def = &set_def.as_ref().unwrap();
                    def.borrow_access()
                }
                _ => panic!("Not a set."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the Set accessor within the variable as mutable.
    /// Panics if unable to do so.
    pub fn set_mut(&mut self) -> &mut Set {
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Set(set_def) => {
                    let def = set_def.as_mut().unwrap();
                    def.mut_access()
                }
                _ => panic!("Not a set."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the Map accessor within the variable.
    /// Panics if unable to do so.
    pub fn map(&self) -> &Map {
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Map(map_def) => {
                    let def = &map_def.as_ref().unwrap();
                    def.borrow_access()
                }
                _ => panic!("Not a map."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Extracts and returns the Map accessor within the variable as mutable.
    /// Panics if unable to do so.
    pub fn map_mut(&mut self) -> &mut Map {
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Map(map_def) => {
                    let def = map_def.as_mut().unwrap();
                    def.mut_access()
                }
                _ => panic!("Not a map."),
            },
            _ => panic!("Not a primitive."),
        }
    }

    /// Attempts to clone the variable, returning a new Variable with the same data specification and value.
    pub fn try_clone(&self) -> Result<Variable, SetEqualToError> {
        // Attempt to clone the variable
        let mut new_variable = default_data_provider().variable_for(self.data_spec());
        new_variable.set_equal_to(self)?;
        Ok(new_variable)
    }
}

impl Default for Variable {
    fn default() -> Self {
        Variable {
            data_spec: DataSpec::default(),
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        // Check if the specification types are equal
        match (
            &self.data_spec().specification_type(),
            &other.data_spec().specification_type(),
        ) {
            (DataSpecType::None, DataSpecType::None) => true,
            (DataSpecType::Primitive(p1), DataSpecType::Primitive(p2)) => match (p1, p2) {
                (Primitive::Integer(i1), Primitive::Integer(i2)) => {
                    let a1 = i1.as_ref().unwrap().borrow_access();
                    let a2 = i2.as_ref().unwrap().borrow_access();
                    a1 == a2
                }
                (Primitive::Float(f1), Primitive::Float(f2)) => {
                    let a1 = f1.as_ref().unwrap().borrow_access();
                    let a2 = f2.as_ref().unwrap().borrow_access();
                    a1 == a2
                }
                (Primitive::Boolean(b1), Primitive::Boolean(b2)) => {
                    let a1 = b1.as_ref().unwrap().borrow_access();
                    let a2 = b2.as_ref().unwrap().borrow_access();
                    a1 == a2
                }
                (Primitive::List(l1), Primitive::List(l2)) => {
                    let a1 = l1.as_ref().unwrap().borrow_access();
                    let a2 = l2.as_ref().unwrap().borrow_access();
                    a1 == a2
                }
                (Primitive::Set(s1), Primitive::Set(s2)) => {
                    let a1 = s1.as_ref().unwrap().borrow_access();
                    let a2 = s2.as_ref().unwrap().borrow_access();
                    a1 == a2
                }
                // (Primitive::Map(m1), Primitive::Map(m2)) => {
                //     let a1 = m1.as_ref().unwrap().borrow_access();
                //     let a2 = m2.as_ref().unwrap().borrow_access();
                //     a1 == a2
                // }
                _ => false,
            },
            _ => false,
        }
    }
}

impl Eq for Variable {}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Compare the data specifications first
        match (
            self.data_spec.specification_type(),
            other.data_spec.specification_type(),
        ) {
            (DataSpecType::Primitive(p1), DataSpecType::Primitive(p2)) => match (p1, p2) {
                (Primitive::Integer(i1), Primitive::Integer(i2)) => {
                    let a1 = i1.as_ref().unwrap().borrow_access();
                    let a2 = i2.as_ref().unwrap().borrow_access();
                    Some(a1.cmp(a2))
                }
                (Primitive::Float(f1), Primitive::Float(f2)) => {
                    let a1 = f1.as_ref().unwrap().borrow_access();
                    let a2 = f2.as_ref().unwrap().borrow_access();
                    Some(a1.cmp(a2))
                }
                (Primitive::Boolean(b1), Primitive::Boolean(b2)) => {
                    let a1 = b1.as_ref().unwrap().borrow_access();
                    let a2 = b2.as_ref().unwrap().borrow_access();
                    Some(a1.cmp(a2))
                }
                (Primitive::List(l1), Primitive::List(l2)) => {
                    let a1 = l1.as_ref().unwrap().borrow_access();
                    let a2 = l2.as_ref().unwrap().borrow_access();
                    Some(a1.cmp(a2))
                }
                (Primitive::Set(s1), Primitive::Set(s2)) => {
                    let a1 = s1.as_ref().unwrap().borrow_access();
                    let a2 = s2.as_ref().unwrap().borrow_access();
                    Some(a1.cmp(a2))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Use the PartialOrd implementation to compare
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Hash for Variable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the value based on its type
        match self.data_spec.specification_type() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Integer(integer_def) => {
                    let def = integer_def.as_ref().unwrap();
                    def.borrow_access().hash(state);
                }
                Primitive::Float(float_def) => {
                    let def = float_def.as_ref().unwrap();
                    def.borrow_access().hash(state);
                }
                Primitive::Boolean(boolean_def) => {
                    let def = boolean_def.as_ref().unwrap();
                    def.borrow_access().hash(state);
                }
                Primitive::List(list_def) => {
                    let def = list_def.as_ref().unwrap();
                    def.borrow_access().hash(state);
                }
                Primitive::Set(set_def) => {
                    let def = set_def.as_ref().unwrap();
                    def.borrow_access().hash(state);
                }
                // Primitive::Map(map_def) => {
                //     let def = map_def.as_ref().unwrap();
                //     def.borrow_access().hash(state);
                // }
                _ => {
                    todo!("Implement hash to for other primitive types");
                }
            },
            _ => {}
        }
    }
}

impl SetEqualTo for Variable {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        // Check if the data specifications are compatible
        self.data_spec.check_compatible_with(other.data_spec())?;

        // Set the variable equal to the other variable
        match self.data_spec.specification_type_mut() {
            DataSpecType::Primitive(primitive) => match primitive {
                Primitive::Integer(integer_def) => {
                    let def = integer_def.as_mut().unwrap();
                    def.mut_access().set_equal_to(other.integer())?;
                }
                Primitive::Float(float_def) => {
                    let def = float_def.as_mut().unwrap();
                    def.mut_access().set_equal_to(other.float())?;
                }
                Primitive::Boolean(boolean_def) => {
                    let def = boolean_def.as_mut().unwrap();
                    def.mut_access().set_equal_to(other.boolean())?;
                }
                Primitive::List(list_def) => {
                    let def = list_def.as_mut().unwrap();
                    def.mut_access().set_equal_to(other.list())?;
                }
                Primitive::Set(set_def) => {
                    let def = set_def.as_mut().unwrap();
                    def.mut_access().set_equal_to(other.set())?;
                }
                Primitive::Map(map_def) => {
                    let def = map_def.as_mut().unwrap();
                    def.mut_access().set_equal_to(other.map())?;
                }
                _ => {
                    todo!("Implement setting equal to for other primitive types");
                }
            },
            _ => {
                panic!("Cannot set equal to a variable with non-primitive data spec.");
            }
        }
        Ok(())
    }
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone Variable")
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.data_spec.specification_type() {
                DataSpecType::Primitive(primitive) => match primitive {
                    Primitive::Integer(_) => self.integer().to_string(),
                    Primitive::Float(_) => self.float().to_string(),
                    Primitive::Boolean(_) => self.boolean().to_string(),
                    Primitive::List(_) => self.list().to_string(),
                    Primitive::Set(_) => self.set().to_string(),
                    //Primitive::Map(_) => self.map().to_string(),
                    _ => "Unsupported primitive".to_string(),
                },
                _ => "Unsupported data spec type".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "spec: {}, value: {})",
            self.data_spec(),
            self.to_string()
        )
    }
}

impl From<DataSpec> for Variable {
    fn from(data_spec: DataSpec) -> Self {
        Variable::new(&data_spec)
    }
}
