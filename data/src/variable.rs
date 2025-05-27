use crate::{
    accessors::{boolean::Boolean, float::Float, integer::Integer},
    data_spec::{DataSpec, DataSpecLevel, DataSpecType},
    primitive::Primitive,
};
//use crate::data_specification::DataSpecLevel;

/// A Variable holds, and provides access to primitives.
pub struct Variable {
    data_spec: DataSpec,
}

impl Variable {
    /// Returns an initialized Variable based on a given primitive.
    pub fn new(primitive: Primitive) -> Variable {
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
}
