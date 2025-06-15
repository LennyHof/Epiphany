use std::{fmt::Debug, rc::Rc};

use crate::{set_equal_to::SetEqualTo, spec_compatibility::SpecCompatibility};

/// A trait for primitive specifications.
pub trait PrimitiveSpec: SpecCompatibility {}

/// A trait for primitive accessors.
pub trait Accessor: SetEqualTo {}

/// PropertyDef pairs a primitive spec and a primitive accessor.
#[derive(Debug, PartialEq)]
pub struct PrimitiveDef<S, A>
where
    S: PrimitiveSpec,
    A: Accessor,
{
    spec: Rc<S>,
    access: Option<A>,
}

impl<S, A> PrimitiveDef<S, A>
where
    S: PrimitiveSpec,
    A: Accessor,
{
    /// Returns an initialized PrimitiveDef.
    pub fn new(spec: Rc<S>, access: Option<A>) -> PrimitiveDef<S, A> {
        PrimitiveDef {
            spec: (spec),
            access: (access),
        }
    }

    /// Returns the property's specification.
    pub fn spec(&self) -> &Rc<S> {
        &self.spec
    }

    /// Returns the property's access.
    pub fn access(&self) -> &Option<A> {
        &self.access
    }

    /// Returns the property's access as a borrow.
    /// Panics if the access is None.
    pub fn borrow_access(&self) -> &A {
        self.access.as_ref().unwrap()
    }

    /// Returns the property's access as a mut.
    /// Panics if the access is None.
    pub fn mut_access(&mut self) -> &mut A {
        self.access.as_mut().unwrap()
    }
}

// Do I need a Variable primitive spec and accessor? Seems needed for Property's accessor.
// Could be needed for recursive data structures?
// I've already done something similar for data specs.
// The Variable would have to conform to the property's primitive spec.
