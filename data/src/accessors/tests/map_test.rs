use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, map_spec_builder::MapSpecBuilder,
    },
    primitive_specs::{integer_spec::IntegerStorage, map_spec::MapKeyOrdering},
    set_equal_to::SetEqualTo,
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
    let mut keys = map.keys();
    let first = keys.next().unwrap().unwrap();
    let second = keys.next().unwrap().unwrap();
    assert!(
        first < second,
        "Keys are not in order: {} < {}",
        first,
        second
    );
}

#[test]
fn map_iterators() {
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
        .set_key_ordering(MapKeyOrdering::Ordered)
        .build();

    let key1 = Variable::try_from(42i64).unwrap();
    let value1 = Variable::try_from(84i64).unwrap();
    let key2 = Variable::try_from(43i64).unwrap();
    let value2 = Variable::try_from(85i64).unwrap();

    let mut map_var = Variable::new(&map_spec);
    {
        let map = map_var.map_mut();

        // Insert two key-value pairs
        assert!(map.insert(key1.clone(), value1.clone()).unwrap());
        assert!(map.insert(key2.clone(), value2.clone()).unwrap());
    }

    {
        // Iterate over an immutable map and check the key-value pairs
        let map = map_var.map();
        let mut iter = map.iter();
        assert_eq!(iter.next().unwrap().unwrap(), (&key1, &value1));
        assert_eq!(iter.next().unwrap().unwrap(), (&key2, &value2));
        assert!(iter.next().is_none());
    }
    {
        // Iterate over a mutable map and check the key-value pairs
        let mut iter = map_var.map_mut().iter();
        assert_eq!(iter.next().unwrap().unwrap(), (&key1, &value1));
        assert_eq!(iter.next().unwrap().unwrap(), (&key2, &value2));
        assert!(iter.next().is_none());
    }
    {
        // Iterate over the keys of the map
        let map = map_var.map();
        let mut keys_iter = map.keys();
        assert_eq!(keys_iter.next().unwrap().unwrap(), &key1);
        assert_eq!(keys_iter.next().unwrap().unwrap(), &key2);
        assert!(keys_iter.next().is_none());
    }
    {
        // Iterate over the values of the map
        let map = map_var.map();
        let mut values_iter = map.values();
        assert_eq!(values_iter.next().unwrap().unwrap(), &value1);
        assert_eq!(values_iter.next().unwrap().unwrap(), &value2);
        assert!(values_iter.next().is_none());
    }
    {
        // Iterate over mutable values of the map
        let mut values_iter_mut = map_var.map_mut().values_mut();
        assert_eq!(values_iter_mut.next().unwrap().unwrap(), &value1.clone());
        assert_eq!(
            values_iter_mut.next().unwrap().unwrap(),
            &mut value2.clone()
        );
        assert!(values_iter_mut.next().is_none());
    }
    {
        // Iterate over the map using IntoIterator
        let map = map_var.map();
        let mut into_iter = map.into_iter();
        assert_eq!(into_iter.next().unwrap().unwrap(), (&key1, &value1));
        assert_eq!(into_iter.next().unwrap().unwrap(), (&key2, &value2));
        assert!(into_iter.next().is_none());
    }
    {
        // Iterate over a mutable map using IntoIterator
        let mut into_iter_mut = map_var.map_mut().into_iter();
        assert_eq!(
            into_iter_mut.next().unwrap().unwrap(),
            (&key1, &mut value1.clone())
        );
        assert_eq!(
            into_iter_mut.next().unwrap().unwrap(),
            (&key2, &mut value2.clone())
        );
        assert!(into_iter_mut.next().is_none());
    }
    {
        // Check that the map is still valid after all iterations
        let map = map_var.map();
        assert_eq!(map.get(&key1).unwrap(), Some(value1));
        assert_eq!(map.get(&key2).unwrap(), Some(value2));
        assert!(!map.is_empty());
        assert!(map.len() == 2);
    }
    {
        // mutate the map using iter_mut and check the values
        let map = map_var.map_mut();
        for entry in map.iter_mut() {
            let (key, value) = entry.unwrap();
            if key == &key1 {
                *value = Variable::try_from(100i64).unwrap();
            } else if key == &key2 {
                *value = Variable::try_from(200i64).unwrap();
            }
        }
        assert!(!map.is_empty());
        assert!(map.len() == 2);
        assert_eq!(
            map.get(&key1).unwrap(),
            Some(Variable::try_from(100i64).unwrap())
        );
        assert_eq!(
            map.get(&key2).unwrap(),
            Some(Variable::try_from(200i64).unwrap())
        );
    }
    {
        // mutate the map using values_mut and check the values
        let map = map_var.map_mut();
        for value in map.values_mut() {
            let value = value.unwrap();
            if value.integer().i64().unwrap() == 100i64 {
                value.integer_mut().set_i64(200i64).unwrap();
            } else if value.integer().i64().unwrap() == 200i64 {
                value.integer_mut().set_i64(300i64).unwrap();
            }
        }
        assert!(!map.is_empty());
        assert!(map.len() == 2);
        assert_eq!(
            map.get(&key1).unwrap(),
            Some(Variable::try_from(200i64).unwrap())
        );
        assert_eq!(
            map.get(&key2).unwrap(),
            Some(Variable::try_from(300i64).unwrap())
        );
    }
}

#[test]
fn map_partial_eq_and_hash() {
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
        .set_key_ordering(MapKeyOrdering::Ordered)
        .build();

    let mut map1 = Variable::new(&map_spec);
    let mut map2 = Variable::new(&map_spec);

    let key1 = Variable::try_from(42i64).unwrap();
    let value1 = Variable::try_from(84i64).unwrap();
    let key2 = Variable::try_from(43i64).unwrap();
    let value2 = Variable::try_from(85i64).unwrap();

    // Insert key-value pairs into both maps
    assert!(map1.map_mut().insert(key1.clone(), value1.clone()).unwrap());
    assert!(map1.map_mut().insert(key2.clone(), value2.clone()).unwrap());
    assert!(map2.map_mut().insert(key1.clone(), value1.clone()).unwrap());
    assert!(map2.map_mut().insert(key2.clone(), value2.clone()).unwrap());

    // Check that the maps are equal
    assert_eq!(map1, map2);
    {
        let mut hasher1 = DefaultHasher::new();
        map1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        map2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // Modify one of the maps
    map1.map_mut()
        .insert(
            Variable::try_from(44i64).unwrap(),
            Variable::try_from(88i64).unwrap(),
        )
        .unwrap();

    // Check that the maps are no longer equal
    assert_ne!(map1, map2);
    {
        let mut hasher1 = DefaultHasher::new();
        map1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        map2.hash(&mut hasher2);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}

#[test]
fn set_equal_to() {
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
        .set_key_ordering(MapKeyOrdering::Ordered)
        .build();

    let mut map1 = Variable::new(&map_spec);

    let key1 = Variable::try_from(42i64).unwrap();
    let value1 = Variable::try_from(84i64).unwrap();
    let key2 = Variable::try_from(43i64).unwrap();
    let value2 = Variable::try_from(85i64).unwrap();

    // Insert key-value pairs into both maps
    assert!(map1.map_mut().insert(key1.clone(), value1.clone()).unwrap());
    assert!(map1.map_mut().insert(key2.clone(), value2.clone()).unwrap());

    // Create a new map and set it equal to map1
    let mut map2 = Variable::new(&map_spec);
    map2.map_mut().set_equal_to(&map1.map()).unwrap();

    // Check that map3 is equal to map1
    assert_eq!(map1, map2);
}

#[test]
fn map_display_and_debug() {
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
        .set_key_ordering(MapKeyOrdering::Ordered)
        .build();

    let mut map_var = Variable::new(&map_spec);
    assert_eq!(map_var.map().to_string(), "Map {}");
    assert_eq!(format!("{:?}", map_var.map()), "Map {}");
    let map = map_var.map_mut();

    let key1 = Variable::try_from(42i64).unwrap();
    let value1 = Variable::try_from(84i64).unwrap();
    let key2 = Variable::try_from(43i64).unwrap();
    let value2 = Variable::try_from(85i64).unwrap();

    // Insert two key-value pairs
    assert!(map.insert(key1.clone(), value1.clone()).unwrap());
    assert!(map.insert(key2.clone(), value2.clone()).unwrap());

    // Check the display output
    assert_eq!(map_var.map().to_string(), "Map {42: 84, 43: 85}");
    // Check the debug output
    assert_eq!(format!("{:?}", map_var.map()), "Map {42: 84, 43: 85}");
}
