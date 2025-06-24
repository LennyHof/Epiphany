use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use crate::{
    accessors::collections::map::{
        MapError, MapIter, MapIterMut, MapKeysIter, MapValuesIter, MapValuesIterMut,
    },
    adaptors::collection_adaptors::map_adaptor::MapAdaptor,
    primitive_specs::map_spec::{MapKeyOrdering, MapSpec},
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

    fn iter_mut<'a>(&'a mut self) -> Box<dyn MapIterMut<'a> + 'a> {
        Box::new(ElementIterMut::new(&mut self.items))
    }

    fn iter<'a>(&'a self) -> Box<dyn MapIter<'a> + 'a> {
        Box::new(ElementIter::new(&self.items))
    }

    fn keys<'a>(&'a self) -> Box<dyn MapKeysIter<'a> + 'a> {
        Box::new(KeyIter::new(&self.items))
    }

    fn values<'a>(&'a self) -> Box<dyn MapValuesIter<'a> + 'a> {
        Box::new(ValueIter::new(&self.items))
    }

    fn values_mut<'a>(&'a mut self) -> Box<dyn MapValuesIterMut<'a> + 'a> {
        Box::new(ValueIterMut::new(&mut self.items))
    }
}

enum MapValueIter<'a> {
    HashMap(std::collections::hash_map::Values<'a, Variable, Variable>),
    BTreeMap(std::collections::btree_map::Values<'a, Variable, Variable>),
}

struct ValueIter<'a> {
    iter: MapValueIter<'a>,
}

impl<'a> ValueIter<'a> {
    fn new(items: &'a MapCollection) -> Self {
        Self {
            iter: match items {
                MapCollection::HashMap(map) => MapValueIter::HashMap(map.values()),
                MapCollection::BTreeMap(map) => MapValueIter::BTreeMap(map.values()),
            },
        }
    }
}

impl<'a> std::iter::Iterator for ValueIter<'a> {
    type Item = Result<&'a Variable, MapError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            MapValueIter::HashMap(iter) => iter.next().map(Ok),
            MapValueIter::BTreeMap(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a> MapValuesIter<'a> for ValueIter<'a> {}

enum MapValueIterMut<'a> {
    HashMap(std::collections::hash_map::ValuesMut<'a, Variable, Variable>),
    BTreeMap(std::collections::btree_map::ValuesMut<'a, Variable, Variable>),
}

struct ValueIterMut<'a> {
    iter: MapValueIterMut<'a>,
}

impl<'a> ValueIterMut<'a> {
    fn new(items: &'a mut MapCollection) -> Self {
        Self {
            iter: match items {
                MapCollection::HashMap(map) => MapValueIterMut::HashMap(map.values_mut()),
                MapCollection::BTreeMap(map) => MapValueIterMut::BTreeMap(map.values_mut()),
            },
        }
    }
}

impl<'a> std::iter::Iterator for ValueIterMut<'a> {
    type Item = Result<&'a mut Variable, MapError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            MapValueIterMut::HashMap(iter) => iter.next().map(Ok),
            MapValueIterMut::BTreeMap(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a> MapValuesIterMut<'a> for ValueIterMut<'a> {}

enum MapKeyIter<'a> {
    HashMap(std::collections::hash_map::Keys<'a, Variable, Variable>),
    BTreeMap(std::collections::btree_map::Keys<'a, Variable, Variable>),
}

struct KeyIter<'a> {
    iter: MapKeyIter<'a>,
}

impl<'a> KeyIter<'a> {
    fn new(items: &'a MapCollection) -> Self {
        Self {
            iter: match items {
                MapCollection::HashMap(map) => MapKeyIter::HashMap(map.keys()),
                MapCollection::BTreeMap(map) => MapKeyIter::BTreeMap(map.keys()),
            },
        }
    }
}

impl<'a> std::iter::Iterator for KeyIter<'a> {
    type Item = Result<&'a Variable, MapError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            MapKeyIter::HashMap(iter) => iter.next().map(Ok),
            MapKeyIter::BTreeMap(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a> MapKeysIter<'a> for KeyIter<'a> {}

enum MapElementIter<'a> {
    HashMap(std::collections::hash_map::Iter<'a, Variable, Variable>),
    BTreeMap(std::collections::btree_map::Iter<'a, Variable, Variable>),
}

struct ElementIter<'a> {
    iter: MapElementIter<'a>,
}

impl<'a> ElementIter<'a> {
    fn new(items: &'a MapCollection) -> Self {
        Self {
            iter: match items {
                MapCollection::HashMap(map) => MapElementIter::HashMap(map.iter()),
                MapCollection::BTreeMap(map) => MapElementIter::BTreeMap(map.iter()),
            },
        }
    }
}

impl<'a> std::iter::Iterator for ElementIter<'a> {
    type Item = Result<(&'a Variable, &'a Variable), MapError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            MapElementIter::HashMap(iter) => iter.next().map(Ok),
            MapElementIter::BTreeMap(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a> MapIter<'a> for ElementIter<'a> {}

enum MapElementIterMut<'a> {
    HashMap(std::collections::hash_map::IterMut<'a, Variable, Variable>),
    BTreeMap(std::collections::btree_map::IterMut<'a, Variable, Variable>),
}

struct ElementIterMut<'a> {
    iter: MapElementIterMut<'a>,
}

impl<'a> ElementIterMut<'a> {
    fn new(items: &'a mut MapCollection) -> Self {
        Self {
            iter: match items {
                MapCollection::HashMap(map) => MapElementIterMut::HashMap(map.iter_mut()),
                MapCollection::BTreeMap(map) => MapElementIterMut::BTreeMap(map.iter_mut()),
            },
        }
    }
}

impl<'a> std::iter::Iterator for ElementIterMut<'a> {
    type Item = Result<(&'a Variable, &'a mut Variable), MapError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            MapElementIterMut::HashMap(iter) => iter.next().map(Ok),
            MapElementIterMut::BTreeMap(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a> MapIterMut<'a> for ElementIterMut<'a> {}
