use crate::{
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_specs::{
        integer_spec::IntegerStorage,
        set_spec::{SetElementOrdering, SetSpec},
    },
    spec_compatibility::SpecCompatibility,
};

#[test]
fn set_no_value_spec_no_element_ordering() {
    let spec = SetSpec::new(&None, None);
    assert!(spec.value_spec().is_none());
    assert!(spec.element_ordering().is_none());
    assert!(spec.is_compatible_with(&SetSpec::new(&None, None)));
    assert!(!spec.is_compatible_with(&SetSpec::new(&None, Some(SetElementOrdering::Unordered))));
    assert!(!spec.is_compatible_with(&SetSpec::new(&None, Some(SetElementOrdering::Ordered))));
}

#[test]
fn set_with_value_spec_no_element_ordering() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpec::new(&Some(value_spec.clone()), None);
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert!(spec.element_ordering().is_none());
    assert!(spec.is_compatible_with(&SetSpec::new(&Some(value_spec.clone()), None)));
    assert!(!spec.is_compatible_with(&SetSpec::new(&None, Some(SetElementOrdering::Unordered))));
}

#[test]
fn unordered_set() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Unordered),
    );
    assert_eq!(
        *spec.element_ordering(),
        Some(SetElementOrdering::Unordered)
    );
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert!(spec.is_compatible_with(&SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Unordered)
    )));
    assert!(!spec.is_compatible_with(&SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Ordered)
    )));
}

#[test]
fn ordered_set() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Ordered),
    );
    assert_eq!(*spec.element_ordering(), Some(SetElementOrdering::Ordered));
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert!(spec.is_compatible_with(&SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Ordered)
    )));
    assert!(!spec.is_compatible_with(&SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Unordered)
    )));
}

#[test]
fn set_element_ordering_display() {
    assert_eq!(SetElementOrdering::Unordered.to_string(), "Unordered");
    assert_eq!(SetElementOrdering::Ordered.to_string(), "Ordered");
}

#[test]
fn set_display() {
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    // unordered set
    let spec = SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Unordered),
    );
    assert_eq!(
        spec.to_string(),
        "Set { value_spec: Integer { encoding: Signed, storage: B64 }, element_ordering: Unordered }"
    );
    // ordered set
    let spec = SetSpec::new(
        &Some(value_spec.clone()),
        Some(SetElementOrdering::Ordered),
    );
    assert_eq!(
        spec.to_string(),
        "Set { value_spec: Integer { encoding: Signed, storage: B64 }, element_ordering: Ordered }"
    );
}
