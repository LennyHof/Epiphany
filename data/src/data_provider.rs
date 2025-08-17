use std::rc::Rc;

use crate::{
    accessors::{
        blob::Blob,
        boolean::Boolean,
        collections::{list::List, map::Map, set::Set},
        float::Float,
        integer::Integer,
        sequence::Sequence,
        strings::{
            byte_string::ByteString, utf8_string::Utf8String, utf16_string::Utf16String,
            utf32_string::Utf32String,
        },
        tuple::Tuple,
    },
    adaptors::{
        blob_adaptor::BlobAdaptor,
        boolean_adaptor::BooleanAdaptor,
        collection_adaptors::{
            list_adaptor::ListAdaptor, map_adaptor::MapAdaptor, set_adaptor::SetAdaptor,
        },
        float_adaptor::FloatAdaptor,
        integer_adaptor::IntegerAdaptor,
        sequence_adaptor::SequenceAdaptor,
        string_adaptors::{
            byte_string_adaptor::ByteStringAdaptor, utf8_string_adaptor::Utf8StringAdaptor,
            utf16_string_adaptor::Utf16StringAdaptor, utf32_string_adaptor::Utf32StringAdaptor,
        },
        tuple_adaptor::TupleAdaptor,
    },
    data_spec::{DataSpec, DataSpecLevel, DataSpecType},
    default_providers::default_data_provider::transient_data_provider::TransientDataProvider,
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::{
        blob_spec::BlobSpec, boolean_spec::BooleanSpec, float_spec::FloatSpec,
        integer_spec::IntegerSpec, list_spec::ListSpec, map_spec::MapSpec,
        sequence_spec::SequenceSpec, set_spec::SetSpec, string_spec::StringSpec,
        tuple_spec::TupleSpec,
    },
    variable::Variable,
};

/// `DataProvider` is a trait for all data providers, which provide adaptors for various accessor types.
///
/// Its function `variable_for` is used to create variables embedding accessors that can access
/// data according to the provided specifications and the specific adaptors provided by the trait's implementation.
pub trait DataProvider {
    /// Returns the provider's name.
    fn name(&self) -> String;

    /// Returns a variable that provides access according to the provided spec.
    fn variable_for(&self, spec: &DataSpec) -> Variable {
        if spec.specification_level() != DataSpecLevel::Access {
            panic!(
                "Specification level must be Access, but was: {}",
                spec.specification_level()
            );
        }
        match spec.specification_type() {
            DataSpecType::Primitive(primitive) => self.variable_for_primitive(primitive),
            _ => panic!("Not a specification for a primitive."),
        }
    }

    /// Returns a variable that provides access according to the provided primitive.
    fn variable_for_primitive(&self, primitive: &Primitive) -> Variable {
        match primitive {
            Primitive::Integer(integer_def) => {
                let int_spec = integer_def.as_ref().unwrap().spec();
                let accessor = Integer::new(self.integer_adaptor(int_spec));
                let def = Some(PrimitiveDef::new(int_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Integer(def))
            }
            Primitive::Float(float_def) => {
                let float_spec = float_def.as_ref().unwrap().spec();
                let accessor = Float::new(self.float_adaptor(float_spec));
                let def = Some(PrimitiveDef::new(float_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Float(def))
            }
            Primitive::Boolean(boolean_def) => {
                let boolean_spec = boolean_def.as_ref().unwrap().spec();
                let accessor = Boolean::new(self.boolean_adaptor(boolean_spec));
                let def = Some(PrimitiveDef::new(boolean_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Boolean(def))
            }
            Primitive::ByteString(string_def) => {
                let string_spec = string_def.as_ref().unwrap().spec();
                let accessor = ByteString::new(self.byte_string_adaptor(string_spec));
                let def = Some(PrimitiveDef::new(string_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::ByteString(def))
            }
            Primitive::Utf8String(string_def) => {
                let string_spec = string_def.as_ref().unwrap().spec();
                let accessor = Utf8String::new(self.utf_8_string_adaptor(string_spec));
                let def = Some(PrimitiveDef::new(string_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Utf8String(def))
            }
            Primitive::Utf16String(string_def) => {
                let string_spec = string_def.as_ref().unwrap().spec();
                let accessor = Utf16String::new(self.utf_16_string_adaptor(string_spec));
                let def = Some(PrimitiveDef::new(string_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Utf16String(def))
            }
            Primitive::Utf32String(string_def) => {
                let string_spec = string_def.as_ref().unwrap().spec();
                let accessor = Utf32String::new(self.utf_32_string_adaptor(string_spec));
                let def = Some(PrimitiveDef::new(string_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Utf32String(def))
            }
            Primitive::Blob(blob_def) => {
                let blob_spec = blob_def.as_ref().unwrap().spec();
                let accessor = Blob::new(self.blob_adaptor(blob_spec));
                let def = Some(PrimitiveDef::new(blob_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Blob(def))
            }
            Primitive::List(list_def) => {
                let list_spec = list_def.as_ref().unwrap().spec();
                let accessor = List::new(self.list_adaptor(list_spec));
                let def = Some(PrimitiveDef::new(list_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::List(def))
            }
            Primitive::Set(set_def) => {
                let set_spec = set_def.as_ref().unwrap().spec();
                let accessor = Set::new(self.set_adaptor(set_spec));
                let def = Some(PrimitiveDef::new(set_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Set(def))
            }
            Primitive::Map(map_def) => {
                let map_spec = map_def.as_ref().unwrap().spec();
                let accessor = Map::new(self.map_adaptor(map_spec));
                let def = Some(PrimitiveDef::new(map_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Map(def))
            }
            Primitive::Sequence(sequence_def) => {
                let sequence_spec = sequence_def.as_ref().unwrap().spec();
                let accessor = Sequence::new(self.sequence_adaptor(sequence_spec));
                let def = Some(PrimitiveDef::new(sequence_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Sequence(def))
            }
            Primitive::Tuple(tuple_def) => {
                let tuple_spec = tuple_def.as_ref().unwrap().spec();
                let accessor = Tuple::new(self.tuple_adaptor(tuple_spec));
                let def = Some(PrimitiveDef::new(tuple_spec.clone(), Some(accessor)));
                Variable::new_primitive(Primitive::Tuple(def))
            }
            _ => todo!(),
        }
    }

    /// Returns a Boolean adaptor according to the given spec.
    fn boolean_adaptor(&self, _spec: &Rc<BooleanSpec>) -> Box<dyn BooleanAdaptor> {
        panic!(
            "Booleans are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns an blob adaptor according to the given spec.
    fn blob_adaptor(&self, _spec: &Rc<BlobSpec>) -> Box<dyn BlobAdaptor> {
        panic!(
            "Blobs are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns an integer adaptor according to the given spec.
    fn integer_adaptor(&self, _spec: &Rc<IntegerSpec>) -> Box<dyn IntegerAdaptor> {
        panic!(
            "Integers are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a float adaptor according to the given spec.
    fn float_adaptor(&self, _spec: &Rc<FloatSpec>) -> Box<dyn FloatAdaptor> {
        panic!(
            "Floats are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a byte string adaptor according to the given spec.
    fn byte_string_adaptor(&self, _spec: &Rc<StringSpec>) -> Box<dyn ByteStringAdaptor> {
        panic!(
            "Byte strings are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a UTF-8 string adaptor according to the given spec.
    fn utf_8_string_adaptor(&self, _spec: &Rc<StringSpec>) -> Box<dyn Utf8StringAdaptor> {
        panic!(
            "UTF-8 strings are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a UTF-16 string adaptor according to the given spec.
    fn utf_16_string_adaptor(&self, _spec: &Rc<StringSpec>) -> Box<dyn Utf16StringAdaptor> {
        panic!(
            "UTF-16 strings are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a UTF-32 string adaptor according to the given spec.
    fn utf_32_string_adaptor(&self, _spec: &Rc<StringSpec>) -> Box<dyn Utf32StringAdaptor> {
        panic!(
            "UTF-32 strings are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a list adaptor according to the given spec.
    fn list_adaptor(&self, _spec: &Rc<ListSpec>) -> Box<dyn ListAdaptor> {
        panic!(
            "Lists are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a set adaptor according to the given spec.
    fn set_adaptor(&self, _spec: &Rc<SetSpec>) -> Box<dyn SetAdaptor> {
        panic!(
            "Sets are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a map adaptor according to the given spec.
    fn map_adaptor(&self, _spec: &Rc<MapSpec>) -> Box<dyn MapAdaptor> {
        panic!(
            "Maps are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a sequence adaptor according to the given spec.
    fn sequence_adaptor(&self, _spec: &Rc<SequenceSpec>) -> Box<dyn SequenceAdaptor> {
        panic!(
            "Sequences are not supported by the {} data provider",
            self.name()
        );
    }

    /// Returns a tuple adaptor according to the given spec.
    fn tuple_adaptor(&self, _spec: &Rc<TupleSpec>) -> Box<dyn TupleAdaptor> {
        panic!(
            "Tuples are not supported by the {} data provider",
            self.name()
        );
    }
}

// /// Registry for data providers.
// pub struct DataProviderRegistry {
//     providers: Mutex<HashMap<String, Rc<dyn DataProvider>>>,
// }

// impl DataProviderRegistry {
//     pub(crate) fn new() -> DataProviderRegistry {
//         DataProviderRegistry {
//             providers: Mutex::new(HashMap::new()),
//         }
//     }

//     /// Adds a data provider to the registry.
//     pub fn add_provider(&self, provider: Rc<dyn DataProvider>) {
//         let mut providers = self.providers.lock().unwrap();
//         providers.insert(provider.name(), provider);
//     }

//     /// Returns the map of registered data providers.
//     pub fn providers(&self) -> &Mutex<HashMap<String, Rc<dyn DataProvider>>> {
//         &self.providers
//     }
// }

/// Returns the default data provider.
pub fn default_data_provider() -> &'static impl DataProvider {
    static TRANSIENT_PROVIDER: TransientDataProvider = TransientDataProvider {};
    &TRANSIENT_PROVIDER
}

// lazy_static! {
//     static DATA_PROVIDER_REGISTRY: DataProviderRegistry = DataProviderRegistry::new();

// }
