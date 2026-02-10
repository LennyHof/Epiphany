use crate::primitive_specs::boolean_spec::BooleanSpec;
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn boolean_spec_new() {
    let _spec = BooleanSpec::new();
}

#[test]
fn boolean_spec_default() {
    let spec = BooleanSpec::default();
    assert!(spec.is_compatible_with(&BooleanSpec::new()));
}

#[test]
fn boolean_spec_compatible() {
    let spec = BooleanSpec::new();
    assert!(spec.is_compatible_with(&BooleanSpec::new()));
}

#[test]
fn boolean_spec_to_string() {
    let spec = BooleanSpec::new();
    assert_eq!(spec.to_string(), "Boolean");
}
