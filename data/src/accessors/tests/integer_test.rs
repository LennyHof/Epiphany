use crate::{
    accessors::integer::IntegerError,
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage},
    set_equal_to::SetEqualTo,
    variable::Variable,
};

#[test]
fn u8_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B8)
            .build(),
    );
    let in_value: u64 = 50u64;
    let int = var.integer_mut();
    int.set_u64(in_value).unwrap();
    assert_eq!(int.u64().unwrap(), in_value);
}

#[test]
fn i8_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B8)
            .build(),
    );
    let in_value: i64 = -50i64;
    let int = var.integer_mut();
    int.set_i64(in_value).unwrap();
    assert_eq!(int.i64().unwrap(), in_value);
}

#[test]
fn u16_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B16)
            .build(),
    );
    let in_value: u64 = 5001u64;
    let int = var.integer_mut();
    int.set_u64(in_value).unwrap();
    assert_eq!(int.u64().unwrap(), in_value);
}

#[test]
fn i16_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B16)
            .build(),
    );
    let in_value: i64 = -5001i64;
    let int = var.integer_mut();
    int.set_i64(in_value).unwrap();
    assert_eq!(int.i64().unwrap(), in_value);
}

#[test]
fn u32_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B32)
            .build(),
    );
    let in_value: u64 = 5001u64;
    let int = var.integer_mut();
    int.set_u64(in_value).unwrap();
    assert_eq!(int.u64().unwrap(), in_value);
}

#[test]
fn i32_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B32)
            .build(),
    );
    let in_value: i64 = -5001i64;
    let int = var.integer_mut();
    int.set_i64(in_value).unwrap();
    assert_eq!(int.i64().unwrap(), in_value);
}

#[test]
fn u64_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B64)
            .build(),
    );
    let in_value: u64 = 5001u64;
    let int = var.integer_mut();
    int.set_u64(in_value).unwrap();
    assert_eq!(int.u64().unwrap(), in_value);
}

#[test]
fn i64_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B64)
            .build(),
    );
    let in_value: i64 = -5001i64;
    let int = var.integer_mut();
    int.set_i64(in_value).unwrap();
    assert_eq!(int.i64().unwrap(), in_value);
}

#[test]
fn from_u8_test() {
    let in_value = 50u8;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: u8 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_i8_test() {
    let in_value = -50i8;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: i8 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_u16_test() {
    let in_value = 50u16;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: u16 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_i16_test() {
    let in_value = -50i16;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: i16 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_u32_test() {
    let in_value = 50u32;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: u32 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_i32_test() {
    let in_value = -50i32;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: i32 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_u64_test() {
    let in_value = 50u64;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: u64 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn from_i64_test() {
    let in_value = -50i64;
    let var: Variable = Variable::try_from(in_value).unwrap();
    let out_value: i64 = var.try_into().unwrap();
    assert_eq!(out_value, in_value);
}

#[test]
fn u8_overflow_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B8)
            .build(),
    );
    assert_eq!(
        var.integer_mut().set_u64(300u64).unwrap_err(),
        IntegerError::Overflow("Value 300 is out of B8 storage range of 0 to 255.".to_string())
    );
}
#[test]
fn i8_overflow_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B8)
            .build(),
    );
    let neg_value: i64 = -300i64;
    let pos_value: i64 = 300i64;
    let int = var.integer_mut();
    assert_eq!(
        int.set_i64(neg_value).unwrap_err(),
        IntegerError::Overflow("Value -300 is out of B8 storage range of -128 to 127.".to_string())
    );
    assert_eq!(
        int.set_i64(pos_value).unwrap_err(),
        IntegerError::Overflow("Value 300 is out of B8 storage range of -128 to 127.".to_string())
    );
}
#[test]
fn u16_overflow_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B16)
            .build(),
    );
    assert_eq!(
        var.integer_mut().set_u64(70000u64).unwrap_err(),
        IntegerError::Overflow(
            "Value 70000 is out of B16 storage range of 0 to 65535.".to_string()
        )
    );
}
#[test]
fn i16_overflow_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B16)
            .build(),
    );
    let neg_value: i64 = -70000i64;
    let pos_value: i64 = 70000i64;
    let int = var.integer_mut();
    assert_eq!(
        int.set_i64(neg_value).unwrap_err(),
        IntegerError::Overflow(
            "Value -70000 is out of B16 storage range of -32768 to 32767.".to_string()
        )
    );
    assert_eq!(
        int.set_i64(pos_value).unwrap_err(),
        IntegerError::Overflow(
            "Value 70000 is out of B16 storage range of -32768 to 32767.".to_string()
        )
    );
}
#[test]
fn u32_overflow_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Unsigned)
            .set_storage(IntegerStorage::B32)
            .build(),
    );
    assert_eq!(
        var.integer_mut().set_u64(70000000000u64).unwrap_err(),
        IntegerError::Overflow(
            "Value 70000000000 is out of B32 storage range of 0 to 4294967295.".to_string()
        )
    );
}
#[test]
fn i32_overflow_test() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B32)
            .build(),
    );
    let neg_value: i64 = -70000000000i64;
    let pos_value: i64 = 70000000000i64;
    let int = var.integer_mut();
    assert_eq!(
        int.set_i64(neg_value).unwrap_err(),
        IntegerError::Overflow(
            "Value -70000000000 is out of B32 storage range of -2147483648 to 2147483647."
                .to_string()
        )
    );
    assert_eq!(
        int.set_i64(pos_value).unwrap_err(),
        IntegerError::Overflow(
            "Value 70000000000 is out of B32 storage range of -2147483648 to 2147483647."
                .to_string()
        )
    );
}

#[test]
fn test_display() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B64)
            .build(),
    );
    let int = var.integer_mut();

    int.set_i64(50).unwrap();
    assert_eq!(format!("{}", int), "50");

    int.set_i64(-50).unwrap();
    assert_eq!(format!("{}", int), "-50");
}
#[test]
fn test_debug() {
    let mut var = Variable::new(
        &IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B64)
            .build(),
    );
    let int = var.integer_mut();

    int.set_i64(50).unwrap();
    assert_eq!(format!("{:?}", int), "50");

    int.set_i64(-50).unwrap();
    assert_eq!(format!("{:?}", int), "-50");
}
#[test]
fn test_partial_eq() {
    let spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert_eq!(var1.integer().i64().unwrap(), 0);
    assert_eq!(var2.integer().i64().unwrap(), 0);
    assert_eq!(var1.integer(), var2.integer());
    assert_eq!(var1, var2);

    var1.integer_mut().set_i64(100).unwrap();
    assert_ne!(var1.integer(), var2.integer());
    assert_ne!(var1, var2);

    var2.integer_mut().set_i64(100).unwrap();
    assert_eq!(var1.integer(), var2.integer());
    assert_eq!(var1, var2);
}
#[test]
fn set_equal_to_i64() {
    let spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert_eq!(var1.integer().i64().unwrap(), 0);
    assert_eq!(var2.integer().i64().unwrap(), 0);
    assert_eq!(var1.integer(), var2.integer());
    assert_eq!(var1, var2);

    var1.integer_mut().set_i64(50).unwrap();
    var2.integer_mut().set_equal_to(var1.integer()).unwrap();
    assert_eq!(var1.integer().i64().unwrap(), 50);
    assert_eq!(var2.integer().i64().unwrap(), 50);
    assert_eq!(var1.integer(), var2.integer());
    assert_eq!(var1, var2);
}
#[test]
fn set_equal_to_u64() {
    let spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Unsigned)
        .set_storage(IntegerStorage::B64)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    assert_eq!(var1.integer().u64().unwrap(), 0);
    assert_eq!(var2.integer().u64().unwrap(), 0);
    assert_eq!(var1.integer(), var2.integer());
    assert_eq!(var1, var2);

    var1.integer_mut().set_u64(50).unwrap();
    var2.integer_mut().set_equal_to(var1.integer()).unwrap();
    assert_eq!(var1.integer().u64().unwrap(), 50);
    assert_eq!(var2.integer().u64().unwrap(), 50);
    assert_eq!(var1.integer(), var2.integer());
    assert_eq!(var1, var2);
}
