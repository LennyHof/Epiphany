use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for edges.
pub struct Edge {}

impl SetEqualTo for Edge {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this Edge equal to another Edge.
        todo!("Implement set_equal_to for Edge");
    }
}

impl Accessor for Edge {}
