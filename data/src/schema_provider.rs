use crate::{accessors::sequence::Sequence, variable::Variable};

/// `SchemaProvider` is a trait for all schema providers, which provide catalogs of object and enum classes.
pub trait SchemaProvider: Send + Sync {
    /// Returns the name of the schema provider.
    fn name(&self) -> String;

    /// Returns the description of the schema provider.
    fn description(&self) -> String;

    /// Returns the version of the schema provider.
    fn version(&self) -> String;

    /// Returns the classes provided by the schema provider.
    fn classes(&self) -> Sequence;

    /// Returns the enum classes provided by the schema provider.
    fn enum_classes(&self) -> Sequence;

    /// This tells the schema provider to represent the class, meaning that add it
    /// if it does not yet exist and to update it if needed if it does.
    fn represent_class(&self, class: Variable);

    /// This tells the schema provider to represent the enum class, meaning that add it
    /// if it does not yet exist and to update it if needed if it does.
    fn represent_enum_class(&self, class: Variable);
}
