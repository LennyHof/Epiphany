use std::rc::Rc;

use crate::{
    adaptors::{
        blob_adaptor::BlobAdaptor, boolean_adaptor::BooleanAdaptor, float_adaptor::FloatAdaptor,
        integer_adaptor::IntegerAdaptor,
    },
    data_provider::DataProvider,
    default_providers::{
        transient_map_adaptor::TransientMapAdaptor, transient_set_adaptor::TransientSetAdaptor,
    },
    primitive_specs::{
        blob_spec::BlobSpec, boolean_spec::BooleanSpec, float_spec::FloatSpec,
        integer_spec::IntegerSpec,
    },
};

use super::{
    transient_boolean_adaptor::TransientBooleanAdaptor,
    transient_float_adaptor::TransientFloatAdaptor,
    transient_integer_adaptor::TransientIntegerAdaptor,
    transient_list_adaptor::TransientListAdaptor,
};

pub struct TransientDataProvider {}

impl DataProvider for TransientDataProvider {
    fn name(&self) -> String {
        "Transient".to_string()
    }

    fn integer_adaptor(&self, spec: &Rc<IntegerSpec>) -> Box<dyn IntegerAdaptor> {
        Box::new(TransientIntegerAdaptor::new(spec.clone()))
    }

    fn float_adaptor(&self, spec: &Rc<FloatSpec>) -> Box<dyn FloatAdaptor> {
        Box::new(TransientFloatAdaptor::new(spec.clone()))
    }

    fn boolean_adaptor(&self, _spec: &Rc<BooleanSpec>) -> Box<dyn BooleanAdaptor> {
        Box::new(TransientBooleanAdaptor::new())
    }
    fn blob_adaptor(&self, _spec: &Rc<BlobSpec>) -> Box<dyn BlobAdaptor> {
        todo!()
    }
    fn list_adaptor(
        &self,
        spec: &Rc<crate::primitive_specs::list_spec::ListSpec>,
    ) -> Box<dyn crate::adaptors::collection_adaptors::list_adaptor::ListAdaptor> {
        Box::new(TransientListAdaptor::new(spec.clone()))
    }
    fn set_adaptor(
        &self,
        spec: &Rc<crate::primitive_specs::set_spec::SetSpec>,
    ) -> Box<dyn crate::adaptors::collection_adaptors::set_adaptor::SetAdaptor> {
        Box::new(TransientSetAdaptor::new(spec.clone()))
    }
    fn map_adaptor(
        &self,
        spec: &Rc<crate::primitive_specs::map_spec::MapSpec>,
    ) -> Box<dyn crate::adaptors::collection_adaptors::map_adaptor::MapAdaptor> {
        Box::new(TransientMapAdaptor::new(spec.clone()))
    }
    fn byte_string_adaptor(
        &self,
        _spec: &Rc<crate::primitive_specs::string_spec::StringSpec>,
    ) -> Box<dyn crate::adaptors::string_adaptors::byte_string_adaptor::ByteStringAdaptor> {
        todo!()
    }
    fn utf_8_string_adaptor(
        &self,
        _spec: &Rc<crate::primitive_specs::string_spec::StringSpec>,
    ) -> Box<dyn crate::adaptors::string_adaptors::utf8_string_adaptor::Utf8StringAdaptor> {
        todo!()
    }
    fn utf_16_string_adaptor(
        &self,
        _spec: &Rc<crate::primitive_specs::string_spec::StringSpec>,
    ) -> Box<dyn crate::adaptors::string_adaptors::utf16_string_adaptor::Utf16StringAdaptor> {
        todo!()
    }
    fn utf_32_string_adaptor(
        &self,
        _spec: &Rc<crate::primitive_specs::string_spec::StringSpec>,
    ) -> Box<dyn crate::adaptors::string_adaptors::utf32_string_adaptor::Utf32StringAdaptor> {
        todo!()
    }
}
