use crate::{
    accessors::{
        collections::map::{self, Map},
        integer::Integer,
    },
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, map_spec_builder::MapSpecBuilder,
    },
    primitive_specs::{integer_spec::IntegerStorage, map_spec::MapKeyOrdering},
    variable::Variable,
};

#[test]
fn map_insert() {
    let map_spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let mut map_var = Variable::new(&map_spec);
    let map = map_var.map_mut();

    let key = Variable::try_from(42i64).unwrap();
    let value = Variable::try_from(84i64).unwrap();

    assert!(map.insert(key.clone(), value.clone()).unwrap());
    assert_eq!(map.get(&key).unwrap(), Some(value));
}

#[test]
fn map_insert_overwrite() {
    let map_spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let mut map_var = Variable::new(&map_spec);
    let map = map_var.map_mut();

    // Check that the map is initially empty
    assert!(map.is_empty());
    assert!(map.len() == 0);

    // Insert a key-value pair
    let key = Variable::try_from(42i64).unwrap();
    let value1 = Variable::try_from(84i64).unwrap();
    let value2 = Variable::try_from(168i64).unwrap();

    assert!(map.insert(key.clone(), value1.clone()).unwrap());
    // check that the map is not empty
    assert!(!map.is_empty());
    assert!(map.len() == 1);
    // Check that the value is correctly inserted
    assert_eq!(map.get(&key).unwrap(), Some(value1));

    // Overwrite the value
    assert!(!map.insert(key.clone(), value2.clone()).unwrap());
    assert_eq!(map.get(&key).unwrap(), Some(value2));

    // Check that the map still has one entry
    assert!(!map.is_empty());
    assert!(map.len() == 1);
}

#[test]
fn map_get_non_existent_key() {
    let map_spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let map_var = Variable::new(&map_spec);
    let map = map_var.map();

    let key = Variable::try_from(42i64).unwrap();
    assert_eq!(map.get(&key).unwrap(), None);
}

#[test]
fn map_remove() {
    let map_spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let mut map_var = Variable::new(&map_spec);
    let map = map_var.map_mut();

    // Check that the map is initially empty
    assert!(map.is_empty());
    assert!(map.len() == 0);

    let key = Variable::try_from(42i64).unwrap();
    let value = Variable::try_from(84i64).unwrap();

    // Insert a key-value pair
    assert!(map.insert(key.clone(), value.clone()).unwrap());
    assert_eq!(map.get(&key).unwrap(), Some(value));

    // Check that the map is not empty
    assert!(!map.is_empty());
    assert!(map.len() == 1);

    // Remove the key
    assert!(map.remove(&key).unwrap());
    assert_eq!(map.get(&key).unwrap(), None);

    // Check that the map is empty again
    assert!(map.is_empty());
    assert!(map.len() == 0);
}

#[test]
fn map_remove_non_existent_key() {
    let map_spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let mut map_var = Variable::new(&map_spec);
    let map = map_var.map_mut();

    let key = Variable::try_from(42i64).unwrap();
    assert!(!map.remove(&key).unwrap());
}

#[test]
fn map_clear() {
    let map_spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let mut map_var = Variable::new(&map_spec);
    let map = map_var.map_mut();

    // Check that the map is initially empty
    assert!(map.is_empty());
    assert!(map.len() == 0);

    let key1 = Variable::try_from(42i64).unwrap();
    let value1 = Variable::try_from(84i64).unwrap();
    let key2 = Variable::try_from(43i64).unwrap();
    let value2 = Variable::try_from(85i64).unwrap();

    // Insert two key-value pairs
    assert!(map.insert(key1.clone(), value1.clone()).unwrap());
    assert!(map.insert(key2.clone(), value2.clone()).unwrap());

    // Check that the map is not empty
    assert!(!map.is_empty());
    assert!(map.len() == 2);

    // Clear the map
    map.clear().unwrap();

    // Check that the map is empty again
    assert!(map.is_empty());
    assert!(map.len() == 0);
}

#[test]
#[ignore = "reason: This test is ignored for now due to anticipated map iter changes."]
/// Tests the ordered map functionality by inserting keys out of order and checking the order of keys.
fn ordered_map() {
    let spec = MapSpecBuilder::new()
        .set_key_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_key_ordering(MapKeyOrdering::Ordered)
        .build();

    let mut map_var = Variable::new(&spec);
    let map = map_var.map_mut();

    // Insert some key-value pairs out of order
    assert!(
        map.insert(
            Variable::try_from(2i64).unwrap(),
            Variable::try_from(20i64).unwrap()
        )
        .unwrap()
    );
    assert!(
        map.insert(
            Variable::try_from(1i64).unwrap(),
            Variable::try_from(10i64).unwrap()
        )
        .unwrap()
    );

    // Check that the values are correctly inserted
    assert_eq!(
        map.get(&Variable::try_from(1i64).unwrap()).unwrap(),
        Some(Variable::try_from(10i64).unwrap())
    );
    assert_eq!(
        map.get(&Variable::try_from(2i64).unwrap()).unwrap(),
        Some(Variable::try_from(20i64).unwrap())
    );

    // Check the order of keys
    let mut keys = map.keys().iter().into_iter();
    let first = keys.next().unwrap().unwrap();
    let second = keys.next().unwrap().unwrap();
    assert!(
        first < second,
        "Keys are not in order: {} < {}",
        first,
        second
    );
}
