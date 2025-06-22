use std::rc::Rc;

use crate::{
    accessors::collections::map::Map,
    data_spec::{DataSpec, DataSpecLevel},
    primitive::Primitive,
    primitive_def::PrimitiveDef,
    primitive_specs::map_spec::{MapKeyOrdering, MapSpec},
};

/// Builder for map data specifications.
///
/// # Examples
///
/// Create a map data specification with string key and string value specifications:    
/// ```rust
/// use data::data_spec_builders::{map_spec_builder::MapSpecBuilder, string_spec_builder::StringSpecBuilder};
/// use data::primitive_specs::string_spec::{StringEncoding, StringStorage};  
/// use data::data_spec::DataSpec;
/// use data::primitive_specs::map_spec::MapSpec;
///
/// let key_spec = StringSpecBuilder::new(StringEncoding::Utf8)
///    .set_storage(StringStorage::VariableSize)
///    .build();
/// let value_spec = StringSpecBuilder::new(StringEncoding::Utf8)
///    .set_storage(StringStorage::VariableSize)
///    .build();
/// let map_data_spec = MapSpecBuilder::new()   
///   .set_key_spec(key_spec)
///   .set_value_spec(value_spec)
///   .build();
/// ```
pub struct MapSpecBuilder {
    key_spec: Option<Rc<DataSpec>>,
    value_spec: Option<Rc<DataSpec>>,
    key_ordering: Option<MapKeyOrdering>,
}

impl MapSpecBuilder {
    /// Returns an initialized map spec builder.
    pub fn new() -> MapSpecBuilder {
        MapSpecBuilder {
            key_spec: None,
            value_spec: None,
            key_ordering: None,
        }
    }

    /// Sets the map's key specification.
    pub fn set_key_spec(&mut self, key_spec: Rc<DataSpec>) -> &mut MapSpecBuilder {
        self.key_spec = Some(key_spec);
        self
    }

    /// Sets the map's value specification.
    pub fn set_value_spec(&mut self, value_spec: Rc<DataSpec>) -> &mut MapSpecBuilder {
        self.value_spec = Some(value_spec);
        self
    }

    /// Sets the map's key ordering.
    pub fn set_key_ordering(&mut self, key_ordering: MapKeyOrdering) -> &mut MapSpecBuilder {
        self.key_ordering = Some(key_ordering);
        self
    }

    /// Builds and returns an initialized data specification.
    pub fn build(&self) -> Rc<DataSpec> {
        let primitive_spec = Rc::new(MapSpec::new(
            &self.key_spec,
            &self.value_spec,
            self.key_ordering,
        ));

        let mut primitive_def: Option<PrimitiveDef<MapSpec, Map>> = None;
        if self.key_spec.is_some() || self.value_spec.is_some() {
            primitive_def = Some(PrimitiveDef::new(primitive_spec, None));
        }

        let specification_level = if self.key_spec.is_some()
            && self.key_spec.as_ref().unwrap().specification_level() == DataSpecLevel::Access
            && self.value_spec.is_some()
            && self.value_spec.as_ref().unwrap().specification_level() == DataSpecLevel::Access
        {
            DataSpecLevel::Access
        } else {
            DataSpecLevel::Compare
        };

        Rc::new(DataSpec::new_primitive(
            Primitive::Map(primitive_def),
            specification_level,
        ))
    }
}

impl Default for MapSpecBuilder {
    fn default() -> Self {
        Self::new()
    }
}
