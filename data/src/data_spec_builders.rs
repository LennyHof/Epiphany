/// Data spec builder for integers.
pub mod integer_spec_builder;

/// Data spec builder for floats.
pub mod float_spec_builder;

/// Data spec builder for booleans.
pub mod boolean_spec_builder;

/// Data spec builder for strings.
pub mod string_spec_builder;

// /// Data spec builder for characters.
// pub mod character_spec_builder;

/// Data spec builder for lists.
pub mod list_spec_builder;

/// Data spec builder for sets.
pub mod set_spec_builder;

/// Data spec builder for maps.
pub mod map_spec_builder;

/// Data spec builder for sequences.
pub mod sequence_spec_builder;

#[cfg(test)]
mod tests {
    mod boolean_spec_builder_test;
    mod float_spec_builder_test;
    mod integer_spec_builder_test;
    mod list_spec_builder_test;
    mod map_spec_builder_test;
    mod sequence_spec_builder_test;
    mod set_spec_builder_test;
    mod string_spec_builder_test;
}
