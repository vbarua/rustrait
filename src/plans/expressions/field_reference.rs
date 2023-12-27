use substrait::proto;

// TODO: This is a vastly simplified field reference
#[derive(Debug)]
pub struct FieldReference {
    pub field: i32,
}

impl From<&proto::expression::FieldReference> for FieldReference {
    fn from(value: &proto::expression::FieldReference) -> Self {
        let root_type = value.root_type.as_ref().expect("root_type must be present");
        match root_type {
            // proto::expression::field_reference::RootType::Expression(_) => {}
            proto::expression::field_reference::RootType::RootReference(_) => {}
            // proto::expression::field_reference::RootType::OuterReference(_) => {}
            _ => panic!("root_type must be root reference, found: {:?}", root_type),
        }

        match value
            .reference_type
            .as_ref()
            .expect("reference_type is set")
        {
            proto::expression::field_reference::ReferenceType::DirectReference(
                reference_segment,
            ) => {
                let ref_type = reference_segment
                    .reference_type
                    .as_ref()
                    .expect("reference_type must be set");
                match ref_type {
                    proto::expression::reference_segment::ReferenceType::MapKey(_) => {
                        panic!("cannot handle map key")
                    }
                    proto::expression::reference_segment::ReferenceType::StructField(sf) => {
                        assert!(sf.child.is_none(), "cannot handle nested reference types");
                        FieldReference { field: sf.field }
                    }
                    proto::expression::reference_segment::ReferenceType::ListElement(_) => {
                        panic!("Cannot handle list element")
                    }
                }
            }
            proto::expression::field_reference::ReferenceType::MaskedReference(_) => {
                panic!("cannot handle mask references")
            }
        }
    }
}
