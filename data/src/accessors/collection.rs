use crate::primitive_def::Accessor;

/// Collection provides general-purpose trait for collections of values,
/// and is the super trait for all specialized collection-accessor traits.
pub trait Collection: Accessor {}
