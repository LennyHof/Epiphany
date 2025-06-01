/// The `Blob` accessor.
pub mod blob;
/// The `Boolean` accessor.
pub mod boolean;
/// The `Character` accessor.
pub mod character;
/// An accessor common to all concrete collection accessors.
pub mod collection;
/// All concrete collection accessors.
pub mod collections;
/// The `DataSpec` accessor.
pub mod data_spec;
/// All concrete date-time accessors.
pub mod date_times;
/// The `EnumObject` accessor.
pub mod enum_object;
/// The `Float` accessor.
pub mod float;
/// All concrete graph acessors.
pub mod graph;
/// The Guid accessor.
pub mod guid;
/// The `Identifier` accessor.
pub mod identifier;
/// The `Integer` accessor.
pub mod integer;
/// The `Object` accessor.
pub mod object;
/// The `Reference` accessor.
pub mod reference;
/// All concrete schema accessors.
pub mod schema;
/// The `Sequence` accessor.
pub mod sequence;
/// An accessor common to all concrete string accessors.
pub mod string;
/// All concrete string accessors.
pub mod strings;
/// The `Tuple` accessor.
pub mod tuple;

#[cfg(test)]
mod tests {
    mod boolean_test;
    mod float_test;
    mod integer_test;
}
