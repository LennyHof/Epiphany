use crate::{
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_specs::{
        integer_spec::IntegerStorage,
        set_spec::{SetSpec, SetStorage},
    },
    spec_compatibility::SpecCompatibility,
};

#[test]
fn set_no_element_spec_no_storage() {
    let spec = SetSpec::new(&None, None);
    assert!(spec.element_spec().is_none());
    assert!(spec.storage().is_none());
    assert!(spec.is_compatible_with(&SetSpec::new(&None, None)));
    assert!(!spec.is_compatible_with(&SetSpec::new(&None, Some(SetStorage::Unordered))));
    assert!(!spec.is_compatible_with(&SetSpec::new(&None, Some(SetStorage::Ordered))));
}

#[test]
fn set_with_element_spec_no_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpec::new(&Some(element_spec.clone()), None);
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.storage().is_none());
    assert!(spec.is_compatible_with(&SetSpec::new(&Some(element_spec.clone()), None)));
    assert!(!spec.is_compatible_with(&SetSpec::new(&None, Some(SetStorage::Unordered))));
}
#[test]
fn set_with_unordered_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpec::new(&Some(element_spec.clone()), Some(SetStorage::Unordered));
    assert_eq!(*spec.storage(), Some(SetStorage::Unordered));
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&SetSpec::new(
        &Some(element_spec.clone()),
        Some(SetStorage::Unordered)
    )));
    assert!(!spec.is_compatible_with(&SetSpec::new(
        &Some(element_spec.clone()),
        Some(SetStorage::Ordered)
    )));
}
#[test]
fn set_with_ordered_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpec::new(&Some(element_spec.clone()), Some(SetStorage::Ordered));
    assert_eq!(*spec.storage(), Some(SetStorage::Ordered));
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&SetSpec::new(
        &Some(element_spec.clone()),
        Some(SetStorage::Ordered)
    )));
    assert!(!spec.is_compatible_with(&SetSpec::new(
        &Some(element_spec.clone()),
        Some(SetStorage::Unordered)
    )));
}

#[test]
fn set_storage_display() {
    assert_eq!(SetStorage::Unordered.to_string(), "Unordered");
    assert_eq!(SetStorage::Ordered.to_string(), "Ordered");
}

#[test]
fn set_display() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    // unordered set
    let spec = SetSpec::new(&Some(element_spec.clone()), Some(SetStorage::Unordered));
    assert_eq!(
        spec.to_string(),
        "Set { element_spec: Integer { encoding: Signed, storage: B64 }, storage: Unordered }"
    );
    // ordered set
    let spec = SetSpec::new(&Some(element_spec.clone()), Some(SetStorage::Ordered));
    assert_eq!(
        spec.to_string(),
        "Set { element_spec: Integer { encoding: Signed, storage: B64 }, storage: Ordered }"
    );
}
