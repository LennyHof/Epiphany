use crate::primitive_specs::float_spec::{FloatSpec, FloatStorage};

#[test]
fn float_spec_no_storage() {
    let spec = FloatSpec::new(None);
    assert!(spec.storage().is_none());
    assert!(spec.is_compatible_with(&FloatSpec::new(None)));
    assert!(!spec.is_compatible_with(&FloatSpec::new(Some(FloatStorage::B32))));
    assert!(!spec.is_compatible_with(&FloatSpec::new(Some(FloatStorage::B64))));
}

#[test]
fn float_spec_b32_storage() {
    let spec = FloatSpec::new(Some(FloatStorage::B32));
    assert_eq!(*spec.storage(), Some(FloatStorage::B32));
    assert!(spec.is_compatible_with(&FloatSpec::new(Some(FloatStorage::B32))));
    assert!(!spec.is_compatible_with(&FloatSpec::new(Some(FloatStorage::B64))));
    assert!(spec.is_compatible_with(&FloatSpec::new(None)));
}
#[test]
fn float_spec_b64_storage() {
    let spec = FloatSpec::new(Some(FloatStorage::B64));
    assert_eq!(*spec.storage(), Some(FloatStorage::B64));
    assert!(spec.is_compatible_with(&FloatSpec::new(Some(FloatStorage::B64))));
    assert!(!spec.is_compatible_with(&FloatSpec::new(Some(FloatStorage::B32))));
    assert!(spec.is_compatible_with(&FloatSpec::new(None)));
}
#[test]
fn float_storage_to_string() {
    let spec = FloatSpec::new(Some(FloatStorage::B32));
    assert_eq!(spec.storage().unwrap().to_string(), "B32");
    let spec = FloatSpec::new(Some(FloatStorage::B64));
    assert_eq!(spec.storage().unwrap().to_string(), "B64");
}

#[test]
fn float_spec_to_string() {
    let spec = FloatSpec::new(None);
    assert_eq!(spec.to_string(), "Float { storage: None }");
    let spec = FloatSpec::new(Some(FloatStorage::B32));
    assert_eq!(spec.to_string(), "Float { storage: B32 }");
    let spec = FloatSpec::new(Some(FloatStorage::B64));
    assert_eq!(spec.to_string(), "Float { storage: B64 }");
}
