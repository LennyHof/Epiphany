use std::sync::Arc;

use crate::{
    accessors::strings::{
        byte_string::ByteString, utf8_string::Utf8String, utf16_string::Utf16String,
        utf32_string::Utf32String,
    },
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::string_spec::{StringEncoding, StringSpec, StringStorage},
};

/// A data specification builder for strings.
/// # Examples
/// Create a string data specification with UTF-8 encoding and variable storage:
/// ```rust
/// use data::spec_builders::string_spec_builder::StringSpecBuilder;
/// use data::primitive_specs::string_spec::{StringEncoding, StringStorage};
/// let string_data_spec = StringSpecBuilder::new(StringEncoding::Utf8)
///     .set_storage(StringStorage::VariableSize)
/// .build();
/// ```
///
pub struct StringSpecBuilder {
    /// The encoding of the string.
    encoding: StringEncoding,
    /// The storage of the string.
    storage: Option<StringStorage>,
}

impl StringSpecBuilder {
    /// Returns an initialized StringSpecBuilder.
    pub fn new(encoding: StringEncoding) -> StringSpecBuilder {
        StringSpecBuilder {
            encoding: encoding,
            storage: None,
        }
    }

    /// Sets the string's storage type.
    pub fn set_storage(&mut self, storage: StringStorage) -> &mut StringSpecBuilder {
        self.storage = Some(storage);
        self
    }

    /// Builds and returns an initialized data specification.
    ///
    /// # Panics
    ///
    /// If the fixed-sized storage is combined with UTF-8 or UTF-16 encoding.
    pub fn build(&self) -> Arc<DataSpec> {
        let specification_level = if self.storage.is_some() {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };
        let primitive_spec = Arc::new(StringSpec::new(self.encoding, self.storage));
        match self.encoding {
            StringEncoding::Byte => {
                let primitive_def: Option<PrimitiveDef<StringSpec, ByteString>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Arc::new(DataSpec::new_primitive(
                    Primitive::ByteString(primitive_def),
                    specification_level,
                ))
            }
            StringEncoding::Utf8 => {
                if self.storage.is_some()
                    && matches!(self.storage, Some(StringStorage::FixedSize(_)))
                {
                    panic!("Fixed-size storage is unavailable for UTF-8 strings.");
                }
                let primitive_def: Option<PrimitiveDef<StringSpec, Utf8String>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Arc::new(DataSpec::new_primitive(
                    Primitive::Utf8String(primitive_def),
                    specification_level,
                ))
            }
            StringEncoding::Utf16 => {
                if self.storage.is_some()
                    && matches!(self.storage, Some(StringStorage::FixedSize(_)))
                {
                    panic!("Fixed-size storage is unavailable for UTF-16 strings.");
                }
                let primitive_def: Option<PrimitiveDef<StringSpec, Utf16String>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Arc::new(DataSpec::new_primitive(
                    Primitive::Utf16String(primitive_def),
                    specification_level,
                ))
            }
            StringEncoding::Utf32 => {
                let primitive_def: Option<PrimitiveDef<StringSpec, Utf32String>> =
                    Some(PrimitiveDef::new(primitive_spec, None));
                Arc::new(DataSpec::new_primitive(
                    Primitive::Utf32String(primitive_def),
                    specification_level,
                ))
            }
        }
    }
}
