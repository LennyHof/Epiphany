use crate::primitive_specs::date_spec::DateSpec;
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn date_spec() {
    let spec = DateSpec::new();
    assert!(spec.is_compatible_with(&DateSpec::new()));
}

#[test]
fn date_spec_to_string() {
    let spec = DateSpec::new();
    assert_eq!(spec.to_string(), "Date");
}