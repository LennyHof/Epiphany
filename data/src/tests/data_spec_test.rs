use std::rc::Rc;

use crate::{
    data_spec::{DataSpec, DataSpecLevel, DataSpecType},
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive::Primitive,
    primitive_category::PrimitiveCategory,
    primitive_def::PrimitiveDef,
    primitive_specs::{
        boolean_spec::BooleanSpec,
        integer_spec::{IntegerEncoding, IntegerStorage},
    },
    spec_compatibility::SpecCompatibility,
};

#[test]
fn data_spec_level_to_string() {
    assert_eq!(DataSpecLevel::Compare.to_string(), "Compare");
    assert_eq!(DataSpecLevel::Access.to_string(), "Access");
}
#[test]
fn data_spec_default() {
    let spec = DataSpec::default();
    assert_eq!(spec.specification_level(), DataSpecLevel::Compare);
    assert_eq!(*spec.specification_type(), DataSpecType::None);
}

#[test]
fn data_spec_new_primitive() {
    let primitive_spec = Rc::new(BooleanSpec::new());
    let primitive_def = Some(PrimitiveDef::new(primitive_spec.clone(), None));
    let spec = Rc::new(DataSpec::new_primitive(
        Primitive::Boolean(primitive_def),
        DataSpecLevel::Access,
    ));
    assert_eq!(spec.specification_level(), DataSpecLevel::Access);
    match &spec.specification_type() {
        DataSpecType::Primitive(p) => match p {
            Primitive::Boolean(primitive_def) => {
                assert!(primitive_def.is_some());
                // Check that the primitive definition's spec is the same as the primitive spec
                assert!(Rc::<BooleanSpec>::ptr_eq(
                    &primitive_def.as_ref().unwrap().spec(),
                    &primitive_spec
                ));
            }
            _ => panic!("Expected a boolean primitive type"),
        },
        _ => panic!("Expected a primitive type"),
    }
}

#[test]
fn data_spec_new_primitive_category() {
    let spec = DataSpec::new_primitive_category(PrimitiveCategory::Collection);
    assert_eq!(spec.specification_level(), DataSpecLevel::Compare);
    match &spec.specification_type() {
        DataSpecType::PrimitiveCategory(category) => {
            assert_eq!(category, &PrimitiveCategory::Collection);
        }
        _ => panic!("Expected a primitive category type"),
    }
}
#[test]
fn data_spec_is_compatible_with() {
    let collection1 = DataSpec::new_primitive_category(PrimitiveCategory::Collection);
    let collection2 = DataSpec::new_primitive_category(PrimitiveCategory::Collection);
    let numberic = DataSpec::new_primitive_category(PrimitiveCategory::Numeric);

    assert!(collection1.is_compatible_with(&collection2));
    assert!(!collection1.is_compatible_with(&numberic));

    let compare_integer_spec1 = IntegerSpecBuilder::new().build();
    let compare_integer_spec2 = IntegerSpecBuilder::new().build();
    let access_integer_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();

    assert!(compare_integer_spec1.is_compatible_with(&compare_integer_spec2));
    assert!(!compare_integer_spec1.is_compatible_with(&access_integer_spec));
    assert!(access_integer_spec.is_compatible_with(&compare_integer_spec1));

    assert!(compare_integer_spec1.is_compatible_with(&numberic));
    assert!(!compare_integer_spec1.is_compatible_with(&collection1));
}
#[test]
fn data_spec_to_string() {
    let spec = DataSpec::default();
    assert_eq!(spec.to_string(), "None");

    let primitive_spec = Rc::new(BooleanSpec::new());
    let primitive_def = Some(PrimitiveDef::new(primitive_spec.clone(), None));
    let spec = Rc::new(DataSpec::new_primitive(
        Primitive::Boolean(primitive_def),
        DataSpecLevel::Access,
    ));
    assert_eq!(spec.to_string(), "Boolean");

    let collection_spec = DataSpec::new_primitive_category(PrimitiveCategory::Collection);
    assert_eq!(collection_spec.to_string(), "Collection");
}
