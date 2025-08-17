use crate::schema_provider::SchemaProvider;

pub(crate) struct TransientSchemaProvider {}

impl SchemaProvider for TransientSchemaProvider {
    fn name(&self) -> String {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }

    fn classes(&self) -> crate::accessors::sequence::Sequence {
        todo!()
    }

    fn enum_classes(&self) -> crate::accessors::sequence::Sequence {
        todo!()
    }

    fn represent_class(&self, class: crate::variable::Variable) {
        todo!()
    }

    fn represent_enum_class(&self, class: crate::variable::Variable) {
        todo!()
    }
}
