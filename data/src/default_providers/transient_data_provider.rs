use std::sync::Arc;

use crate::{
    adaptors::{
        blob_adaptor::BlobAdaptor, boolean_adaptor::BooleanAdaptor, float_adaptor::FloatAdaptor,
        integer_adaptor::IntegerAdaptor,
    },
    data_provider::DataProvider,
    primitive_specs::{
        blob_spec::BlobSpec, boolean_spec::BooleanSpec, float_spec::FloatSpec,
        integer_spec::IntegerSpec,
    },
};

use super::{
    transient_boolean_adaptor::TransientBooleanAdaptor,
    transient_float_adaptor::TransientFloatAdaptor,
    transient_integer_adaptor::TransientIntegerAdaptor,
};

pub struct TransientDataProvider {}

impl DataProvider for TransientDataProvider {
    fn name(&self) -> String {
        "Transient".to_string()
    }

    fn integer_adaptor(&self, spec: &Arc<IntegerSpec>) -> Box<dyn IntegerAdaptor> {
        Box::new(TransientIntegerAdaptor::new(spec.clone()))
    }

    fn float_adaptor(&self, spec: &Arc<FloatSpec>) -> Box<dyn FloatAdaptor> {
        Box::new(TransientFloatAdaptor::new(spec.clone()))
    }

    fn boolean_adaptor(&self, _spec: &Arc<BooleanSpec>) -> Box<dyn BooleanAdaptor> {
        Box::new(TransientBooleanAdaptor::new())
    }
    fn blob_adaptor(&self, _spec: &Arc<BlobSpec>) -> Box<dyn BlobAdaptor> {
        todo!()
    }
}
