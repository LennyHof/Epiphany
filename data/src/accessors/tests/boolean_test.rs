use crate::{
    data_provider::{DataProvider, default_data_provider},
    spec_builders::boolean_spec_builder::BooleanSpecBuilder,
    variable::Variable,
};

#[test]
fn test_true() {
    let spec = BooleanSpecBuilder::new().build();
    let value: bool = true;
    let mut var = default_data_provider().variable_for(&spec);
    let boolean = var.boolean_mut();
    boolean.set_boolean(value).unwrap();
    assert_eq!(boolean.boolean().unwrap(), value);
}

#[test]
fn test_false() {
    let spec = BooleanSpecBuilder::new().build();
    let value: bool = false;
    let mut var = default_data_provider().variable_for(&spec);
    let boolean = var.boolean_mut();
    boolean.set_boolean(value).unwrap();
    assert_eq!(boolean.boolean().unwrap(), value);
}

#[test]
fn test_from_bool() {
    let in_value: bool = true;
    let var = Variable::try_from(in_value).unwrap();
    let out_value: bool = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}
