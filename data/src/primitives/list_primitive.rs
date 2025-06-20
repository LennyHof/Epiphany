use std::rc::Rc;

use crate::{accessors::collections::list::List, primitive_specs::list_spec::ListSpec};

/// An ordered collection of potentially non-unique values.
pub struct ListPrimitive {
    /// The list's specification.
    pub spec: Rc<ListSpec>,
    /// The list's storage.
    pub storage: Option<Arc<dyn List>>,
}

impl ListPrimitive {
    /// Returns an initialized ListPrimitive without storage.
    pub fn new(sp: Rc<ListSpec>) -> ListPrimitive {
        ListPrimitive {
            spec: sp.clone(),
            storage: None,
        }
    }

    /// Returns an initialized ListPrimitive with storage.
    // pub fn clone_with_storage(other: &ListPrimitive) -> ListPrimitive {
    //     ListPrimitive{spec: other.spec.clone(), storage: Some(0.0f64)}
    // }

    /// Returns the primitive's specification.
    pub fn spec(&self) -> &ListSpec {
        &self.spec
    }

    /// Returns the primitive's storage.
    pub fn storage(&self) -> &Option<Arc<dyn List>> {
        &self.storage
    }
}
