use crate::{
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_specs::map_spec::MapSpec,
    primitive_specs::{integer_spec::IntegerStorage, map_spec::MapKeyOrdering},
    spec_compatibility::SpecCompatibility,
};

#[test]
fn map_no_key_spec_no_value_spec_no_key_ordering() {
    let spec = MapSpec::new(&None, &None, None);
    assert!(spec.key_spec().is_none());
    assert!(spec.value_spec().is_none());
    assert!(spec.key_ordering().is_none());
    assert!(spec.is_compatible_with(&MapSpec::new(&None, &None, None)));
    assert!(!spec.is_compatible_with(&MapSpec::new(&None, &None, Some(MapKeyOrdering::Unordered))));
    assert!(!spec.is_compatible_with(&MapSpec::new(&None, &None, Some(MapKeyOrdering::Ordered))));
}

#[test]
fn map_with_key_spec_no_value_spec_no_key_ordering() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = MapSpec::new(&Some(key_spec.clone()), &None, None);
    assert_eq!(*spec.key_spec(), Some(key_spec.clone()));
    assert!(spec.value_spec().is_none());
    assert!(spec.key_ordering().is_none());
    assert!(spec.is_compatible_with(&MapSpec::new(&Some(key_spec.clone()), &None, None)));
    assert!(!spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &None,
        Some(MapKeyOrdering::Unordered)
    )));
}
#[test]
fn map_with_key_spec_with_value_spec_no_key_ordering() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = MapSpec::new(&Some(key_spec.clone()), &Some(value_spec.clone()), None);
    assert_eq!(*spec.key_spec(), Some(key_spec.clone()));
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert!(spec.key_ordering().is_none());
    assert!(spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        None
    )));
    assert!(!spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Unordered)
    )));
    assert!(!spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Ordered)
    )));
    assert!(spec.is_compatible_with(&MapSpec::new(&None, &Some(value_spec.clone()), None)));
    assert!(spec.is_compatible_with(&MapSpec::new(&Some(key_spec.clone()), &None, None)));
}

#[test]
fn map_with_key_spec_with_value_spec_with_key_ordering() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Unordered),
    );
    assert_eq!(*spec.key_spec(), Some(key_spec.clone()));
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert_eq!(*spec.key_ordering(), Some(MapKeyOrdering::Unordered));
    assert!(spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Unordered)
    )));
    assert!(!spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Ordered)
    )));
}

#[test]
fn ordered_map() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Ordered),
    );
    assert_eq!(*spec.key_spec(), Some(key_spec.clone()));
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert_eq!(*spec.key_ordering(), Some(MapKeyOrdering::Ordered));
    assert!(spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Ordered)
    )));
    assert!(!spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Unordered)
    )));
}

#[test]
fn unordered_map() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Unordered),
    );
    assert_eq!(*spec.key_spec(), Some(key_spec.clone()));
    assert_eq!(*spec.value_spec(), Some(value_spec.clone()));
    assert_eq!(*spec.key_ordering(), Some(MapKeyOrdering::Unordered));
    assert!(spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Unordered)
    )));
    assert!(!spec.is_compatible_with(&MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Ordered)
    )));
}

#[test]
fn map_key_ordering_display() {
    assert_eq!(MapKeyOrdering::Unordered.to_string(), "Unordered");
    assert_eq!(MapKeyOrdering::Ordered.to_string(), "Ordered");
}
#[test]
fn map_display() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = IntegerSpecBuilder::new()
        .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = MapSpec::new(
        &Some(key_spec.clone()),
        &Some(value_spec.clone()),
        Some(MapKeyOrdering::Ordered),
    );
    assert_eq!(
        spec.to_string(),
        "Map { key_spec: Integer { encoding: Signed, storage: B64 }, value_spec: Integer { encoding: Signed, storage: B64 }, key_ordering: Ordered }"
    );
}
