use crate::{
    data_spec_builders::{
        float_spec_builder::FloatSpecBuilder, integer_spec_builder::IntegerSpecBuilder,
    },
    primitive_def::IsOrdered,
    primitive_specs::tuple_spec::TupleSpec,
    spec_compatibility::SpecCompatibility,
};

#[test]
fn tuple_spec_no_value_specs() {
    let tuple_spec = TupleSpec::new(None);
    assert!(tuple_spec.value_specs().is_none());
}
#[test]
fn tuple_spec_with_value_specs() {
    let int_spec = IntegerSpecBuilder::new().build();
    let float_spec = FloatSpecBuilder::new().build();
    let value_specs = vec![int_spec.clone(), float_spec.clone()];
    let tuple_spec = TupleSpec::new(Some(value_specs));
    assert!(tuple_spec.value_specs().is_some());
    assert_eq!(tuple_spec.value_specs().as_ref().unwrap().len(), 2);
    assert_eq!(tuple_spec.value_specs().as_ref().unwrap()[0], int_spec);
    assert_eq!(tuple_spec.value_specs().as_ref().unwrap()[1], float_spec);
}

#[test]
fn tuple_spec_compatible_with() {
    let int_spec = IntegerSpecBuilder::new().build();
    let float_spec = FloatSpecBuilder::new().build();
    let value_specs = vec![int_spec.clone(), float_spec.clone()];
    let tuple_spec = TupleSpec::new(Some(value_specs.clone()));

    // Compatible with itself
    assert!(tuple_spec.is_compatible_with(&tuple_spec));

    // Compatible with another tuple spec with the same value specs
    let other_tuple_spec = TupleSpec::new(Some(value_specs.clone()));
    assert!(tuple_spec.is_compatible_with(&other_tuple_spec));

    // Not compatible with a tuple spec with different length value specs
    let different_len_specs = vec![int_spec.clone()];
    let other_tuple_spec_different = TupleSpec::new(Some(different_len_specs));
    assert!(!tuple_spec.is_compatible_with(&other_tuple_spec_different));

    // Not compatible with a tuple spec with different value types
    let different_value_specs = vec![float_spec.clone(), int_spec.clone()];
    let other_tuple_spec_different_values = TupleSpec::new(Some(different_value_specs));
    assert!(!tuple_spec.is_compatible_with(&other_tuple_spec_different_values));
}

#[test]
fn tuple_spec_compatible_with_none() {
    let tuple_spec = TupleSpec::new(None);
    // Compatible with another tuple spec with no value specs
    let other_tuple_spec = TupleSpec::new(None);
    assert!(tuple_spec.is_compatible_with(&other_tuple_spec));

    // Compatible with a tuple spec with value specs (assumed compatibility)
    let int_spec = IntegerSpecBuilder::new().build();
    let float_spec = FloatSpecBuilder::new().build();
    let value_specs = vec![int_spec, float_spec];
    let other_tuple_spec_with_values = TupleSpec::new(Some(value_specs));
    assert!(!tuple_spec.is_compatible_with(&other_tuple_spec_with_values));
    assert!(other_tuple_spec.is_compatible_with(&tuple_spec));
}

#[test]
fn tuple_spec_ordered() {
    let tuple_spec = TupleSpec::new(None);
    // Tuples are considered ordered collections
    assert!(tuple_spec.is_ordered());

    let int_spec = IntegerSpecBuilder::new().build();
    let float_spec = FloatSpecBuilder::new().build();
    let value_specs = vec![int_spec, float_spec];
    let ordered_tuple_spec = TupleSpec::new(Some(value_specs));
    assert!(ordered_tuple_spec.is_ordered());
}

#[test]
fn tuple_spec_display() {
    let int_spec = IntegerSpecBuilder::new().build();
    let float_spec = FloatSpecBuilder::new().build();
    let value_specs = vec![int_spec.clone(), float_spec.clone()];
    let tuple_spec = TupleSpec::new(Some(value_specs));

    // Check the display output
    assert_eq!(
        tuple_spec.to_string(),
        "Tuple { value_specs: {Integer, Float} }"
    );

    // Check the display output with no value specs
    let empty_tuple_spec = TupleSpec::new(None);
    assert_eq!(
        empty_tuple_spec.to_string(),
        "Tuple { value_specs: { None } }"
    );
}
