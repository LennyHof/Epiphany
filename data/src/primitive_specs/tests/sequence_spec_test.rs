use crate::{
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_specs::{integer_spec::IntegerStorage, sequence_spec::SequenceSpec},
    spec_compatibility::SpecCompatibility,
};

#[test]
fn sequence_spec_no_value_spec() {
    let spec = SequenceSpec::new(&None);
    assert!(spec.value_spec().is_none());
}
#[test]
fn sequence_spec_with_value_spec() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SequenceSpec::new(&Some(value_spec.clone()));
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
}
#[test]
fn sequence_spec_compatible_with() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SequenceSpec::new(&Some(value_spec.clone()));
    let compatible_spec = SequenceSpec::new(&Some(value_spec.clone()));
    assert!(spec.is_compatible_with(&compatible_spec));
}
#[test]
fn sequence_spec_incompatible_with_different_value_spec() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SequenceSpec::new(&Some(value_spec.clone()));
    let incompatible_value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Unsigned)
        .set_storage(IntegerStorage::B64)
        .build();
    let incompatible_spec = SequenceSpec::new(&Some(incompatible_value_spec));
    assert!(!spec.is_compatible_with(&incompatible_spec));
}
#[test]
fn sequence_spec_incompatible_with_no_value_spec() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let with_element_spec = SequenceSpec::new(&Some(value_spec.clone()));
    let no_element_spec = SequenceSpec::new(&None);
    assert!(with_element_spec.is_compatible_with(&no_element_spec));
    assert!(!no_element_spec.is_compatible_with(&with_element_spec));
}
#[test]
fn sequence_spec_display() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SequenceSpec::new(&Some(value_spec.clone()));
    assert_eq!(
        spec.to_string(),
        format!("Sequence {{ value_spec: {} }}", value_spec)
    );
}
