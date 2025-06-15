use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for references.
pub struct Reference {}

impl SetEqualTo for Reference {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Reference");
    }
}
impl Accessor for Reference {}
