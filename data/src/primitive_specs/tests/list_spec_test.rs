use crate::spec_compatibility::SpecCompatibility;
use crate::{
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_specs::{
        integer_spec::IntegerStorage,
        list_spec::{ListSpec, ListStorage},
    },
};

#[test]
fn list_no_element_spec_no_storage() {
    let spec = ListSpec::new(&None, &None);
    assert!(spec.storage().is_none());
    assert!(spec.element_spec().is_none());
    assert!(spec.is_compatible_with(&ListSpec::new(&None, &None)));
    assert!(!spec.is_compatible_with(&ListSpec::new(&None, &Some(ListStorage::FixedSize(10)))));
    assert!(!spec.is_compatible_with(&ListSpec::new(&None, &Some(ListStorage::FixedCapacity(10)))));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &None,
        &Some(ListStorage::InitialCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(&None, &Some(ListStorage::VariableSize))));
}

#[test]
fn list_with_element_spec_no_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = ListSpec::new(&Some(element_spec.clone()), &None);
    assert!(spec.storage().is_none());
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&ListSpec::new(&Some(element_spec.clone()), &None)));
    assert!(!spec.is_compatible_with(&ListSpec::new(&None, &Some(ListStorage::FixedSize(10)))));
}

#[test]
fn list_with_fixed_size_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedSize(10)),
    );
    assert_eq!(*spec.storage(), Some(ListStorage::FixedSize(10)));
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedSize(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::InitialCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::VariableSize)
    )));
}

#[test]
fn list_with_fixed_capacity_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedCapacity(10)),
    );
    assert_eq!(*spec.storage(), Some(ListStorage::FixedCapacity(10)));
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedSize(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::InitialCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::VariableSize)
    )));
}
#[test]
fn list_with_initial_capacity_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::InitialCapacity(10)),
    );
    assert_eq!(*spec.storage(), Some(ListStorage::InitialCapacity(10)));
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::InitialCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedSize(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::VariableSize)
    )));
}
#[test]
fn list_with_variable_size_storage() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::VariableSize),
    );
    assert_eq!(*spec.storage(), Some(ListStorage::VariableSize));
    assert_eq!(*spec.element_spec(), Some(element_spec.clone()));
    assert!(spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::VariableSize)
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedSize(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedCapacity(10))
    )));
    assert!(!spec.is_compatible_with(&ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::InitialCapacity(10))
    )));
}
#[test]
fn list_spec_to_string() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = ListSpec::new(&Some(element_spec.clone()), &None);
    assert_eq!(
        spec.to_string(),
        "List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: None }"
    );

    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedSize(10)),
    );
    assert_eq!(
        spec.to_string(),
        "List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: FixedSize(10) }"
    );

    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::FixedCapacity(10)),
    );
    assert_eq!(
        spec.to_string(),
        "List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: FixedCapacity(10) }"
    );

    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::InitialCapacity(10)),
    );
    assert_eq!(
        spec.to_string(),
        "List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: InitialCapacity(10) }"
    );

    let spec = ListSpec::new(
        &Some(element_spec.clone()),
        &Some(ListStorage::VariableSize),
    );
    assert_eq!(
        spec.to_string(),
        "List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: VariableSize }"
    );
}
