use crate::primitive_category::PrimitiveCategory;

#[test]
fn logical_type_category_name() {
    assert_eq!(PrimitiveCategory::Numeric.to_string(), "Numeric");
    assert_eq!(PrimitiveCategory::Basic.to_string(), "Basic");
    assert_eq!(PrimitiveCategory::Simple.to_string(), "Simple");
    assert_eq!(
        PrimitiveCategory::ObjectOrReference.to_string(),
        "ObjectOrReference"
    );
    assert_eq!(PrimitiveCategory::Collection.to_string(), "Collection");
    assert_eq!(PrimitiveCategory::Sequenceable.to_string(), "Sequenceable");
    assert_eq!(PrimitiveCategory::Schema.to_string(), "Schema");
    assert_eq!(PrimitiveCategory::String.to_string(), "String");
    assert_eq!(PrimitiveCategory::All.to_string(), "All");
}
