//! Data Library
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

/// The `ProviderError` enum.
pub mod provider_error;

/// The `SpecCompatibility` trait.
pub mod spec_compatibility;

/// The `SetEqualTo` trait.
pub mod set_equal_to;

/// Specifications for specific primitives.
pub mod primitive_specs;

/// The `PrimitiveDef` generic struct.
pub mod primitive_def;

/// The `Primitive` enumeration.
pub mod primitive;

/// The `PrimitiveCategory` enumeration.
pub mod primitive_category;

/// Specs for specific primitive-categories.
pub mod primitive_category_specs;

/// The `DataSpec` struct.
pub mod data_spec;

/// Data spec builders.
pub mod data_spec_builders;

/// All accessors.
pub mod accessors;

/// The `Adaptor` trait.
pub mod adaptor;

/// All adaptors
pub mod adaptors;

/// The `Variable` stuct.
pub mod variable;

/// The `DataProvider` trait.    
pub mod data_provider;

/// The `SchemaProvider` trait.
pub mod schema_provider;

/// The `SchemaProviderRegistry` struct.
/// This is a registry for schema providers, allowing for the association of data providers with their respective   
/// schema providers. It provides a way to manage and retrieve schema providers based on the data providers they are associated with.
/// It is used to ensure that data providers can access the correct schema provider without needing to maintain
/// state about their associated schema provider.
/// This registry is essential for the operation of the data library, as it allows for the dynamic
/// association of data providers with their schema providers, enabling
/// the retrieval of schema information based on the data provider being used.
pub mod schema_provider_registry;

/// Default providers.
pub(crate) mod default_providers;

#[cfg(test)]
mod tests {
    mod data_spec_test;
    mod primitive_test;
    mod primitive_type_category_test;
}
