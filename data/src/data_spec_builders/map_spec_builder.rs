use std::sync::Arc;

use crate::{
    accessors::collections::map::Map,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::map_spec::MapSpec,
};

/// Builder for map data specifications.
///
/// # Examples
///
/// Create a map data specification with string key and string element specifications:    
/// ```rust
/// use data::data_spec_builders::{map_spec_builder::MapSpecBuilder, string_spec_builder::StringSpecBuilder};
/// use data::primitive_specs::string_spec::{StringEncoding, StringStorage};  
/// use data::data_spec::DataSpec;
/// use data::primitive_specs::map_spec::MapSpec;
/// use std::sync::Arc;
///
/// let key_spec = StringSpecBuilder::new(StringEncoding::Utf8)
///    .set_storage(StringStorage::VariableSize)
///    .build();
/// let element_spec = StringSpecBuilder::new(StringEncoding::Utf8)
///    .set_storage(StringStorage::VariableSize)
///    .build();
/// let map_data_spec = MapSpecBuilder::new()   
///   .set_key_spec(key_spec)
///   .set_element_spec(element_spec)
///   .build();
/// ```
pub struct MapSpecBuilder {
    key_spec: Option<Arc<DataSpec>>,
    element_spec: Option<Arc<DataSpec>>,
}
impl MapSpecBuilder {
    /// Returns an initialized map spec builder.
    pub fn new() -> MapSpecBuilder {
        MapSpecBuilder {
            key_spec: None,
            element_spec: None,
        }
    }

    /// Sets the map's key specification.
    pub fn set_key_spec(&mut self, key_spec: Arc<DataSpec>) -> &mut MapSpecBuilder {
        self.key_spec = Some(key_spec);
        self
    }

    /// Sets the map's element specification.
    pub fn set_element_spec(&mut self, element_spec: Arc<DataSpec>) -> &mut MapSpecBuilder {
        self.element_spec = Some(element_spec);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Arc<DataSpec> {
        let primitive_spec = Arc::new(MapSpec::new(&self.key_spec, &self.element_spec));

        let mut primitive_def: Option<PrimitiveDef<MapSpec, Map>> = None;
        if self.key_spec.is_some() || self.element_spec.is_some() {
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
        }

        let specification_level = if self.key_spec.is_some()
            && self.key_spec.as_ref().unwrap().specification_level() == DataSpecLevel::Access
            && self.element_spec.is_some()
            && self.element_spec.as_ref().unwrap().specification_level() == DataSpecLevel::Access
        {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };

        Arc::new(DataSpec::new_primitive(
            Primitive::Map(primitive_def),
            specification_level,
        ))
    }
}
