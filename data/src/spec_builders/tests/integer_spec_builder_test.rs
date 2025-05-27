use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    primitive::Primitive,
    primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage},
    spec_builders::integer_spec_builder::IntegerSpecBuilder,
};

#[test]
fn no_encoding_no_storage() {
    let spec = IntegerSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => {
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
fn signed_no_storage() {
    let spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => match value.spec().encoding() {
                    Some(IntegerEncoding::Signed) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
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
fn unsigned_no_storage() {
    let spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Unsigned)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => match value.spec().encoding() {
                    Some(IntegerEncoding::Unsigned) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
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
fn b8_no_encoding() {
    let spec = IntegerSpecBuilder::new()
        .set_storage(IntegerStorage::B8)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => match value.spec().storage() {
                    Some(IntegerStorage::B8) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
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
fn b16_no_encoding() {
    let spec = IntegerSpecBuilder::new()
        .set_storage(IntegerStorage::B16)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => match value.spec().storage() {
                    Some(IntegerStorage::B16) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
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
fn b32_no_encoding() {
    let spec = IntegerSpecBuilder::new()
        .set_storage(IntegerStorage::B32)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => match value.spec().storage() {
                    Some(IntegerStorage::B32) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
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
fn b64_no_encoding() {
    let spec = IntegerSpecBuilder::new()
        .set_storage(IntegerStorage::B64)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => match value.spec().storage() {
                    Some(IntegerStorage::B64) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
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
fn encoding_and_storage() {
    let spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Unsigned)
        .set_storage(IntegerStorage::B64)
        .build();

    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Integer(def) => match def {
                Some(value) => {
                    match value.spec().encoding() {
                        Some(IntegerEncoding::Unsigned) => {}
                        _ => assert!(false),
                    }
                    match value.spec().storage() {
                        Some(IntegerStorage::B64) => {}
                        _ => assert!(false),
                    }
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}
