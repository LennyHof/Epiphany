use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use crate::{
    accessors::{collections::map::MapError, sequence::SequenceIter},
    adaptors::{
        collection_adaptors::map_adaptor::{self, MapAdaptor},
        sequence_adaptor::SequenceAdaptor,
    },
    primitive_specs::{
        map_spec::{MapKeyOrdering, MapSpec},
        sequence_spec::SequenceSpec,
    },
    variable::Variable,
};

enum MapCollection {
    HashMap(HashMap<Variable, Variable>),
    BTreeMap(BTreeMap<Variable, Variable>),
}

pub struct TransientMapAdaptor {
    spec: Rc<MapSpec>,
    items: MapCollection,
}

impl TransientMapAdaptor {
    pub fn new(spec: Rc<MapSpec>) -> Self {
        Self {
            spec: spec.clone(),
            items: match spec.key_ordering().as_ref() {
                Some(MapKeyOrdering::Unordered) => MapCollection::HashMap(HashMap::new()),
                Some(MapKeyOrdering::Ordered) => MapCollection::BTreeMap(BTreeMap::new()),
                None => MapCollection::HashMap(HashMap::new()), // Default to HashMap if no key_ordering type is specified
            },
        }
    }
}

impl MapAdaptor for TransientMapAdaptor {
    fn spec(&self) -> &Rc<MapSpec> {
        &self.spec
    }

    fn len(&self) -> usize {
        match &self.items {
            MapCollection::HashMap(map) => map.len(),
            MapCollection::BTreeMap(map) => map.len(),
        }
    }

    fn contains(&self, key: &Variable) -> Result<bool, MapError> {
        match &self.items {
            MapCollection::HashMap(map) => Ok(map.contains_key(key)),
            MapCollection::BTreeMap(map) => Ok(map.contains_key(key)),
        }
    }

    fn do_insert(&mut self, key: Variable, value: Variable) -> Result<bool, MapError> {
        match &mut self.items {
            MapCollection::HashMap(map) => Ok(map.insert(key, value).is_none()),
            MapCollection::BTreeMap(map) => Ok(map.insert(key, value).is_none()),
        }
    }

    fn do_remove(&mut self, key: &Variable) -> Result<bool, MapError> {
        match &mut self.items {
            MapCollection::HashMap(map) => Ok(map.remove(key).is_some()),
            MapCollection::BTreeMap(map) => Ok(map.remove(key).is_some()),
        }
    }

    fn get(&self, key: &Variable) -> Result<Option<Variable>, MapError> {
        match &self.items {
            MapCollection::HashMap(map) => Ok(map.get(key).cloned()),
            MapCollection::BTreeMap(map) => Ok(map.get(key).cloned()),
        }
    }

    fn get_mut(&mut self, key: &Variable) -> Result<Option<&mut Variable>, MapError> {
        match &mut self.items {
            MapCollection::HashMap(map) => Ok(map.get_mut(key)),
            MapCollection::BTreeMap(map) => Ok(map.get_mut(key)),
        }
    }

    fn clear(&mut self) -> Result<(), MapError> {
        match &mut self.items {
            MapCollection::HashMap(map) => {
                map.clear();
                Ok(())
            }
            MapCollection::BTreeMap(map) => {
                map.clear();
                Ok(())
            }
        }
    }

    fn elements(&self) -> Box<dyn SequenceAdaptor> {
        todo!("Implement elements sequence for TransientMapAdaptor")
    }

    fn keys(&self) -> Box<dyn SequenceAdaptor> {
        todo!("Implement keys sequence for TransientMapAdaptor")
    }

    fn values(&self) -> Box<dyn SequenceAdaptor> {
        todo!("Implement values sequence for TransientMapAdaptor")
    }
}
