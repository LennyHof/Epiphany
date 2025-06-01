use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::float_spec_builder::FloatSpecBuilder,
    primitive::Primitive,
    primitive_specs::float_spec::FloatStorage,
};

#[test]
fn no_storage() {
    let spec = FloatSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Float(def) => {
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
fn f32_storage() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B32)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Float(def) => match def {
                Some(value) => match value.spec().storage() {
                    Some(FloatStorage::B32) => {}
                    _ => assert!(false),
                },
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
#[test]
fn f64_storage() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B64)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Float(def) => match def {
                Some(value) => match value.spec().storage() {
                    Some(FloatStorage::B64) => {}
                    _ => assert!(false),
                },
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
