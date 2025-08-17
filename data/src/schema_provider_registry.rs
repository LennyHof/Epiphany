use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use crate::schema_provider::SchemaProvider;

static SCHEMA_PROVIDER_REGISTRY: LazyLock<RwLock<SchemaProviderRegistry>> =
    LazyLock::new(|| RwLock::new(SchemaProviderRegistry::new()));

/// `SchemaProviderRegistry` is a registry for schema providers.
pub struct SchemaProviderRegistry {
    providers: HashMap<String, Box<dyn SchemaProvider>>,
    default_provider: Box<dyn SchemaProvider>,
}

impl SchemaProviderRegistry {
    /// Creates a new `SchemaProviderRegistry`.
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: Box::new(crate::default_providers::default_schema_provider::transient_schema_provider::TransientSchemaProvider {}),
        }
    }

    /// Registers a schema provider with the registry.
    pub fn register(&mut self, provider: Box<dyn SchemaProvider>) {
        self.providers.insert(provider.name(), provider);
    }

    /// Retrieves a schema provider by its name.
    pub fn provider(&self, name: &str) -> Option<&Box<dyn SchemaProvider>> {
        self.providers.get(name)
    }

    /// Retrieves the default schema provider.
    pub fn default_provider(&self) -> &Box<dyn SchemaProvider> {
        &self.default_provider
    }
}
