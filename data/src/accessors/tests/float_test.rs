use crate::{
    accessors::float::FloatError,
    data_provider::{DataProvider, default_data_provider},
    primitive_specs::float_spec::FloatStorage,
    data_spec_builders::float_spec_builder::FloatSpecBuilder,
    variable::Variable,
};

#[test]
fn f32_test() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B32)
        .build();
    let value: f64 = 50.0;
    let mut var = default_data_provider().variable_for(&spec);
    let float = var.float_mut();
    float.set_f64(value).unwrap();
    assert_eq!(float.f64().unwrap(), value);
}

#[test]
fn f64_test() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B64)
        .build();
    let value: f64 = 50.0;
    let mut var = default_data_provider().variable_for(&spec);
    let float = var.float_mut();
    float.set_f64(value).unwrap();
    assert_eq!(float.f64().unwrap(), value);
}

#[test]
fn f32_overflow_test() {
    let spec = FloatSpecBuilder::new()
        .set_storage(FloatStorage::B32)
        .build();
    let mut var = default_data_provider().variable_for(&spec);
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
