use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    primitive::Primitive,
    data_spec_builders::boolean_spec_builder::BooleanSpecBuilder,
};

#[test]
fn bool_spec() {
    let spec = BooleanSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Boolean(def) => {
                assert!(def.is_some());
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
