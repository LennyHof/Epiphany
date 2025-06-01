use std::sync::Arc;

use crate::primitive_specs::guid_spec::GuidSpec;

/// An adaptor for GUID values.
pub trait GuidAdaptor {
    /// Returns the GUID's specification.
    fn spec(&self) -> &Arc<GuidSpec>;
}
