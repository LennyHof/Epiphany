use crate::{
    accessors::float::FloatError, data_spec_builders::float_spec_builder::FloatSpecBuilder,
    primitive_specs::float_spec::FloatStorage, set_equal_to::SetEqualTo, variable::Variable,
};

#[test]
fn f32_test() {
    let mut var = Variable::new(
        &FloatSpecBuilder::new()
            .set_storage(FloatStorage::B32)
            .build(),
    );
    let value: f64 = 50.0;
    let float = var.float_mut();
    float.set_f64(value).unwrap();
    assert_eq!(float.f64().unwrap(), value);
}

#[test]
fn f64_test() {
    let mut var = Variable::new(
        &FloatSpecBuilder::new()
            .set_storage(FloatStorage::B64)
            .build(),
    );
    let value: f64 = 50.0;
    let float = var.float_mut();
    float.set_f64(value).unwrap();
    assert_eq!(float.f64().unwrap(), value);
}

#[test]
fn f32_overflow_test() {
    let mut var = Variable::new(
        &FloatSpecBuilder::new()
            .set_storage(FloatStorage::B32)
            .build(),
    );
    let float = var.float_mut();
    let pos_result = float.set_f64(1.0e40);
    assert_eq!(
        pos_result,
        Err(FloatError::Overflow("value 10000000000000000000000000000000000000000 is out of B32 storage range of -340282346638528860000000000000000000000 to 340282346638528860000000000000000000000.".to_string()))
    );
    let neg_result = float.set_f64(-1.0e40);
    assert_eq!(
        neg_result,
        Err(FloatError::Overflow("value -10000000000000000000000000000000000000000 is out of B32 storage range of -340282346638528860000000000000000000000 to 340282346638528860000000000000000000000.".to_string()))
    );
}

#[test]
fn from_f32_test() {
    let in_value: f32 = 50.0;
    let var = Variable::try_from(in_value).unwrap();
    let out_value: f32 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}
#[test]
fn from_f64_test() {
    let in_value: f64 = 50.0;
    let var = Variable::try_from(in_value).unwrap();
    let out_value: f64 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn set_equal_to() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B32)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert_eq!(var1.float().f64().unwrap(), 0.0);
    assert_eq!(var2.float().f64().unwrap(), 0.0);
    assert_eq!(var1.float(), var2.float());
    assert_eq!(var1, var2);

    var1.float_mut().set_f64(50.0).unwrap();
    var2.float_mut().set_equal_to(var1.float()).unwrap();
    assert_eq!(var1.float().f64().unwrap(), 50.0);
    assert_eq!(var2.float().f64().unwrap(), 50.0);
    assert_eq!(var1.float(), var2.float());
    assert_eq!(var1, var2);
}

#[test]
fn test_display() {
    let mut var = Variable::new(
        &FloatSpecBuilder::new()
            .set_storage(FloatStorage::B32)
            .build(),
    );
    let float = var.float_mut();

    float.set_f64(50.0).unwrap();
    assert_eq!(format!("{}", float), "50");

    float.set_f64(0.0).unwrap();
    assert_eq!(format!("{}", float), "0");
}

#[test]
fn test_debug() {
    let mut var = Variable::new(
        &FloatSpecBuilder::new()
            .set_storage(FloatStorage::B32)
            .build(),
    );
    let float = var.float_mut();

    float.set_f64(50.0).unwrap();
    assert_eq!(format!("{:?}", float), "50");

    float.set_f64(0.0).unwrap();
    assert_eq!(format!("{:?}", float), "0");
}

#[test]
fn test_partial_eq() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B32)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert!(var1.float() == var2.float());

    var1.float_mut().set_f64(50.0).unwrap();
    assert!(var1.float() != var2.float());

    var2.float_mut().set_f64(50.0).unwrap();
    assert!(var1.float() == var2.float());
}