use crate::primitive_specs::integer_spec::{IntegerEncoding, IntegerSpec, IntegerStorage};
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn integer_spec_no_encoding_no_storage() {
    let spec = IntegerSpec::new(None, None);
    assert!(spec.storage().is_none());
    assert!(spec.encoding().is_none());
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, None)));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(Some(IntegerEncoding::Signed), None)));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(Some(IntegerEncoding::Unsigned), None)));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B8))));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B16))));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B32))));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B64))));
}

#[test]
fn integer_spec_signed_no_storage() {
    let spec = IntegerSpec::new(Some(IntegerEncoding::Signed), None);
    assert!(spec.storage().is_none());
    assert_eq!(*spec.encoding(), Some(IntegerEncoding::Signed));
    assert!(spec.is_compatible_with(&IntegerSpec::new(Some(IntegerEncoding::Signed), None)));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(Some(IntegerEncoding::Unsigned), None)));
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, None)));
}
#[test]
fn integer_spec_unsigned_no_storage() {
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), None);
    assert!(spec.storage().is_none());
    assert_eq!(*spec.encoding(), Some(IntegerEncoding::Unsigned));
    assert!(spec.is_compatible_with(&IntegerSpec::new(Some(IntegerEncoding::Unsigned), None)));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(Some(IntegerEncoding::Signed), None)));
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, None)));
}

#[test]
fn integer_spec_b8_storage() {
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B8));
    assert_eq!(*spec.encoding(), Some(IntegerEncoding::Unsigned));
    assert_eq!(*spec.storage(), Some(IntegerStorage::B8));
    assert!(spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B8)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B16)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B32)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B64)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Signed),
        Some(IntegerStorage::B8)
    )));
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B8))));
}

#[test]
fn integer_spec_b16_storage() {
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B16));
    assert_eq!(*spec.encoding(), Some(IntegerEncoding::Unsigned));
    assert_eq!(*spec.storage(), Some(IntegerStorage::B16));
    assert!(spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B16)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B8)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B32)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B64)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Signed),
        Some(IntegerStorage::B16)
    )));
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B16))));
}
#[test]
fn integer_spec_b32_storage() {
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B32));
    assert_eq!(*spec.encoding(), Some(IntegerEncoding::Unsigned));
    assert_eq!(*spec.storage(), Some(IntegerStorage::B32));
    assert!(spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B32)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B8)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B16)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B64)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Signed),
        Some(IntegerStorage::B32)
    )));
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B32))));
}
#[test]
fn integer_spec_b64_storage() {
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B64));
    assert_eq!(*spec.encoding(), Some(IntegerEncoding::Unsigned));
    assert_eq!(*spec.storage(), Some(IntegerStorage::B64));
    assert!(spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B64)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B8)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B16)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Unsigned),
        Some(IntegerStorage::B32)
    )));
    assert!(!spec.is_compatible_with(&IntegerSpec::new(
        Some(IntegerEncoding::Signed),
        Some(IntegerStorage::B64)
    )));
    assert!(spec.is_compatible_with(&IntegerSpec::new(None, Some(IntegerStorage::B64))));
}
#[test]
fn integer_storage_to_string() {
    assert_eq!(IntegerStorage::B8.to_string(), "B8");
    assert_eq!(IntegerStorage::B16.to_string(), "B16");
    assert_eq!(IntegerStorage::B32.to_string(), "B32");
    assert_eq!(IntegerStorage::B64.to_string(), "B64");
}
#[test]
fn integer_encoding_to_string() {
    assert_eq!(IntegerEncoding::Signed.to_string(), "Signed");
    assert_eq!(IntegerEncoding::Unsigned.to_string(), "Unsigned");
}

#[test]
fn integer_spec_to_string() {
    let spec = IntegerSpec::new(None, None);
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: None, storage: None }"
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Signed), Some(IntegerStorage::B8));
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Signed, storage: B8 }".to_string()
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B8));
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Unsigned, storage: B8 }".to_string()
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Signed), Some(IntegerStorage::B32));
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Signed, storage: B32 }".to_string()
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B32));
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Unsigned, storage: B32 }".to_string()
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Signed), Some(IntegerStorage::B64));
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Signed, storage: B64 }"
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), Some(IntegerStorage::B16));
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Unsigned, storage: B16 }"
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Signed), None);
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Signed, storage: None }"
    );
    let spec = IntegerSpec::new(Some(IntegerEncoding::Unsigned), None);
    assert_eq!(
        spec.to_string(),
        "Integer { encoding: Unsigned, storage: None }"
    );
    let spec = IntegerSpec::new(None, Some(IntegerStorage::B8));
    assert_eq!(spec.to_string(), "Integer { encoding: None, storage: B8 }");
    let spec = IntegerSpec::new(None, Some(IntegerStorage::B16));
    assert_eq!(spec.to_string(), "Integer { encoding: None, storage: B16 }");
    let spec = IntegerSpec::new(None, Some(IntegerStorage::B32));
    assert_eq!(spec.to_string(), "Integer { encoding: None, storage: B32 }");
    let spec = IntegerSpec::new(None, Some(IntegerStorage::B64));
    assert_eq!(spec.to_string(), "Integer { encoding: None, storage: B64 }");
}
