//! Data Library
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

/// The `ProviderError` enum.
pub mod provider_error;

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

/// Default data provider implementations.
mod default_providers;

#[cfg(test)]
mod tests {
    mod primitive_test;
    mod primitive_type_category_test;
}
