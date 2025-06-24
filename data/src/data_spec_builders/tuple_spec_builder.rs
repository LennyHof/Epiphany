use std::rc::Rc;

use crate::{
    accessors::tuple::Tuple,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::tuple_spec::TupleSpec,
};

/// Builder for tuple data specifications.
///
/// # Examples
///
/// Create a data specification for an tuple of an integer and float:
/// ```rust
/// use data::data_spec_builders::{tuple_spec_builder::TupleSpecBuilder, integer_spec_builder::IntegerSpecBuilder, float_spec_builder::FloatSpecBuilder};
/// use data::primitive_specs::{integer_spec::{IntegerEncoding, IntegerStorage}, float_spec::FloatStorage};
///
/// let tuple_data_spec = TupleSpecBuilder::new()
///     .add_value_spec(IntegerSpecBuilder::new()
///         .set_encoding(IntegerEncoding::Signed)
///         .set_storage(IntegerStorage::B64)
///        .build())
///    .add_value_spec(FloatSpecBuilder::new()
///        .set_storage(FloatStorage::B64)
///       .build())
///    .build();
/// ```
pub struct TupleSpecBuilder {
    value_specs: Option<Vec<Rc<DataSpec>>>,
}

impl TupleSpecBuilder {
    /// Creates a new `TupleSpecBuilder`.
    pub fn new() -> Self {
        Self { value_specs: None }
    }

    /// Adds an value specification to the tuple.
    pub fn add_value_spec(&mut self, value_spec: Rc<DataSpec>) -> &mut Self {
        if let Some(ref mut specs) = self.value_specs {
            specs.push(value_spec);
        } else {
            self.value_specs = Some(vec![value_spec]);
        }
        self
    }

    /// Builds the tuple specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let mut primitive_def: Option<PrimitiveDef<TupleSpec, Tuple>> = None;
        let mut specification_level = DataSpecLevel::Compare;
        if self.value_specs.is_some() {
            // Determine the specification level based on value specs
            specification_level = DataSpecLevel::Access; // Default to Access level
            for value_spec in self.value_specs.as_ref().unwrap() {
                if value_spec.specification_level() == DataSpecLevel::Compare {
                    specification_level = DataSpecLevel::Compare;
                    break; // If any value spec is at Compare level, the whole tuple is at Compare level
                }
            }
            let primitive_spec = Rc::new(TupleSpec::new(self.value_specs.clone()));
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
        }

        Rc::new(DataSpec::new_primitive(
            Primitive::Tuple(primitive_def),
            specification_level,
        ))
    }
}

impl Default for TupleSpecBuilder {
    fn default() -> Self {
        Self::new()
    }
}
