use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, map_spec_builder::MapSpecBuilder,
        string_spec_builder::StringSpecBuilder,
    },
    primitive::Primitive,
    primitive_specs::{
        integer_spec::{IntegerEncoding, IntegerStorage},
        map_spec::MapKeyOrdering,
        string_spec::{StringEncoding, StringStorage},
    },
};

#[test]
fn map_spec_builder_default() {
    let spec = MapSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_none());
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
fn map_spec_builder_with_key_and_element() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = StringSpecBuilder::new(StringEncoding::Utf8)
        .set_storage(StringStorage::VariableSize)
        .build();

    let spec = MapSpecBuilder::new()
        .set_key_spec(key_spec)
        .set_value_spec(value_spec)
        .build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_some());
                let def = def.as_ref().unwrap();
                let map_spec = def.spec().as_ref();
                match map_spec.key_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
                match map_spec.value_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Utf8String(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn map_spec_builder_with_key_lement_and_unordered() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = StringSpecBuilder::new(StringEncoding::Utf8)
        .set_storage(StringStorage::VariableSize)
        .build();

    let spec = MapSpecBuilder::new()
        .set_key_spec(key_spec)
        .set_value_spec(value_spec)
        .set_key_ordering(crate::primitive_specs::map_spec::MapKeyOrdering::Unordered)
        .build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_some());
                let def = def.as_ref().unwrap();
                let map_spec = def.spec().as_ref();
                match map_spec.key_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
                match map_spec.value_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Utf8String(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
                assert_eq!(map_spec.key_ordering(), &Some(MapKeyOrdering::Unordered));
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn map_spec_builder_with_key_only() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();

    let spec = MapSpecBuilder::new().set_key_spec(key_spec).build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_some());
                let def = def.as_ref().unwrap();
                let map_spec = def.spec().as_ref();
                match map_spec.key_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
fn map_spec_builder_with_element_only() {
    let value_spec = StringSpecBuilder::new(StringEncoding::Utf8)
        .set_storage(StringStorage::VariableSize)
        .build();

    let spec = MapSpecBuilder::new().set_value_spec(value_spec).build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_some());
                let def = def.as_ref().unwrap();
                let map_spec = def.spec().as_ref();
                match map_spec.value_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Utf8String(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
fn map_spec_builder_with_key_compare_and_element_access() {
    let key_spec = IntegerSpecBuilder::new().build();
    let value_spec = StringSpecBuilder::new(StringEncoding::Utf8)
        .set_storage(StringStorage::VariableSize)
        .build();

    let spec = MapSpecBuilder::new()
        .set_key_spec(key_spec)
        .set_value_spec(value_spec)
        .build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_some());
                let def = def.as_ref().unwrap();
                let map_spec = def.spec().as_ref();
                match map_spec.key_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
                match map_spec.value_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Utf8String(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
fn map_spec_builder_with_key_access_and_element_compare() {
    let key_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let value_spec = StringSpecBuilder::new(StringEncoding::Utf8).build();

    let spec = MapSpecBuilder::new()
        .set_key_spec(key_spec)
        .set_value_spec(value_spec)
        .build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Map(def) => {
                assert!(def.is_some());
                let def = def.as_ref().unwrap();
                let map_spec = def.spec().as_ref();
                match map_spec.key_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
                match map_spec.value_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Utf8String(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}
