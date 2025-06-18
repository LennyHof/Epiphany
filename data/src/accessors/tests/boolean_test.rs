use crate::{
    data_spec_builders::boolean_spec_builder::BooleanSpecBuilder, set_equal_to::SetEqualTo,
    variable::Variable,
};

#[test]
fn test_true() {
    let mut var = Variable::new(&BooleanSpecBuilder::new().build());
    let boolean = var.boolean_mut();
    let value: bool = true;
    boolean.set_boolean(value).unwrap();
    assert_eq!(boolean.boolean().unwrap(), value);
}

#[test]
fn test_false() {
    let spec = BooleanSpecBuilder::new().build();
    let value: bool = false;
    let mut var = Variable::new(&spec);
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

#[test]
fn set_equal_to() {
    let spec = BooleanSpecBuilder::new().build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert_eq!(var1.boolean().boolean().unwrap(), false);
    assert_eq!(var2.boolean().boolean().unwrap(), false);
    assert_eq!(var1.boolean(), var2.boolean());
    assert_eq!(var1, var2);

    var1.boolean_mut().set_boolean(true).unwrap();
    var2.boolean_mut().set_equal_to(var1.boolean()).unwrap();
    assert_eq!(var1.boolean().boolean().unwrap(), true);
    assert_eq!(var2.boolean().boolean().unwrap(), true);
    assert_eq!(var1.boolean(), var2.boolean());
    assert_eq!(var1, var2);
}

#[test]
fn test_display() {
    let mut var = Variable::new(&BooleanSpecBuilder::new().build());
    let boolean = var.boolean_mut();

    boolean.set_boolean(true).unwrap();
    assert_eq!(format!("{}", boolean), "true");

    boolean.set_boolean(false).unwrap();
    assert_eq!(format!("{}", boolean), "false");
}

#[test]
fn test_debug() {
    let mut var = Variable::new(&BooleanSpecBuilder::new().build());
    let boolean = var.boolean_mut();

    boolean.set_boolean(true).unwrap();
    assert_eq!(format!("{:?}", boolean), "true");

    boolean.set_boolean(false).unwrap();
    assert_eq!(format!("{:?}", boolean), "false");
}

#[test]
fn test_partial_eq() {
    let spec = BooleanSpecBuilder::new().build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert!(var1.boolean() == var2.boolean());

    var1.boolean_mut().set_boolean(true).unwrap();
    assert!(var1.boolean() != var2.boolean());

    var2.boolean_mut().set_boolean(true).unwrap();
    assert!(var1.boolean() == var2.boolean());
}
