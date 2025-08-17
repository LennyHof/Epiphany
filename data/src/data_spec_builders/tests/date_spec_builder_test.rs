use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::date_spec_builder::DateSpecBuilder,
    primitive::Primitive,
};

#[test]
fn build() {
    let spec = DateSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Date(def) => {
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
