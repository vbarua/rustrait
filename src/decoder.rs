use crate::plans::expressions::field_reference::FieldReference;
use crate::plans::expressions::literal::{Bool, Literal, I32, I64};
use crate::plans::expressions::scalar_function::ScalarFunctionInvocation;
use crate::plans::expressions::{Expression, Function, FunctionArgument, FunctionSignature, URI};
use crate::plans::{Plan, Project, Read, Rel};
use crate::types;
use crate::types::{NamedStruct, Type};
use std::collections::HashMap;
use substrait::proto;
use substrait::proto::extensions::simple_extension_declaration::{ExtensionFunction, MappingType};

trait PlanDecoder<PlanFormat> {
    fn decode(&mut self, plan: &PlanFormat) -> Plan;
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct ExtensionAnchor(u32);

#[derive(Debug, Eq, Hash, PartialEq)]
struct FunctionAnchor(u32);

pub fn decode_prost_plan(plan: &proto::Plan) -> Plan {
    let mut decoder = ProstPlanDecoder::new();
    return decoder.decode(plan);
}

pub struct ProstPlanDecoder {
    extension_uri_map: HashMap<ExtensionAnchor, URI>,
    function_map: HashMap<FunctionAnchor, Function>,
}

impl PlanDecoder<proto::Plan> for ProstPlanDecoder {
    fn decode(&mut self, plan: &proto::Plan) -> Plan {
        self.gather_extensions(plan);
        self.decode_plan(plan)
    }
}

impl ProstPlanDecoder {
    fn new() -> ProstPlanDecoder {
        return ProstPlanDecoder {
            extension_uri_map: HashMap::new(),
            function_map: HashMap::new(),
        };
    }

    fn decode_plan(&self, p: &proto::Plan) -> Plan {
        let root = p.relations.get(0).expect("assume 1 rel for now");
        let rel_type = root.rel_type.as_ref().expect("expecting presence of plan");
        return Plan {
            root: Box::new(match rel_type {
                proto::plan_rel::RelType::Rel(rel) => self.decode_relation(rel),
                proto::plan_rel::RelType::Root(rel_root) => {
                    self.decode_relation(rel_root.input.as_ref().expect("argh"))
                }
            }),
        };
    }

    fn decode_relation(&self, rel: &proto::Rel) -> Rel {
        let rel_type = rel.rel_type.as_ref().expect("rel_type must be set");
        match rel_type {
            proto::rel::RelType::Read(rr) => Rel::Read(self.decode_read(rr)),
            // RelType::Filter(_) => {}
            // RelType::Fetch(_) => {}
            // RelType::Aggregate(_) => {}
            // RelType::Sort(_) => {}
            // RelType::Join(_) => {}
            proto::rel::RelType::Project(pr) => Rel::Project(self.decode_project(pr)),
            // RelType::Set(_) => {}
            // RelType::ExtensionSingle(_) => {}
            // RelType::ExtensionMulti(_) => {}
            // RelType::ExtensionLeaf(_) => {}
            // RelType::Cross(_) => {}
            // RelType::Reference(_) => {}
            // RelType::Write(_) => {}
            // RelType::Ddl(_) => {}
            // RelType::HashJoin(_) => {}
            // RelType::MergeJoin(_) => {}
            // RelType::NestedLoopJoin(_) => {}
            // RelType::Window(_) => {}
            // RelType::Exchange(_) => {}
            // RelType::Expand(_) => {}
            _ => panic!("cannot handle rel: {:?}", rel),
        }
    }

    fn decode_read(&self, rr: &proto::ReadRel) -> Read {
        Read {
            base_schema: self
                .decode_named_struct(rr.base_schema.as_ref().expect("base schema must be set")),
        }
    }

    fn decode_project(&self, pr: &proto::ProjectRel) -> Project {
        let input = pr.input.as_ref().expect("project input must be set");
        Project {
            input: Box::new(self.decode_relation(input.as_ref())),
            expressions: pr
                .expressions
                .iter()
                .map(|expr| self.decode_expression(expr))
                .collect(),
        }
    }

    fn decode_named_struct(&self, ns: &proto::NamedStruct) -> NamedStruct {
        NamedStruct {
            names: ns.names.clone(),
            types: ns
                .r#struct
                .as_ref()
                .expect("types should be present")
                .types
                .iter()
                .map(|t| self.decode_type(t))
                .collect(),
        }
    }

    fn decode_expression(&self, expr: &proto::Expression) -> Expression {
        match expr.rex_type.as_ref().expect("rex_type is set") {
            proto::expression::RexType::Literal(literal) => {
                Expression::Literal(self.decode_literal(literal))
            }
            proto::expression::RexType::Selection(field_reference) => {
                Expression::FieldReference(self.decode_field_reference(field_reference.as_ref()))
            }
            proto::expression::RexType::ScalarFunction(scalar_function) => {
                let args: Vec<FunctionArgument> = scalar_function
                    .arguments
                    .iter()
                    .map(|fa| self.decode_function_argument(fa))
                    .collect();

                let function = self
                    .function_map
                    .get(&FunctionAnchor(scalar_function.function_reference))
                    .expect("function was not in plan")
                    .clone();

                let output_type =
                    self.decode_type(scalar_function.output_type.as_ref().expect("no type"));

                Expression::ScalarFunction(ScalarFunctionInvocation {
                    function,
                    args,
                    output_type,
                })
            }
            // RexType::WindowFunction(_) => {}
            // RexType::IfThen(_) => {}
            // RexType::SwitchExpression(_) => {}
            // RexType::SingularOrList(_) => {}
            // RexType::MultiOrList(_) => {}
            // RexType::Cast(_) => {}
            // RexType::Subquery(_) => {}
            // RexType::Nested(_) => {}
            // RexType::Enum(_) => {}
            _ => panic!("cannot handle expression {:?}", expr),
        }
    }

    fn decode_function_argument(&self, fa: &proto::FunctionArgument) -> FunctionArgument {
        let arg_type = fa.arg_type.as_ref().expect("function argument must be set");
        match arg_type {
            proto::function_argument::ArgType::Enum(enu) => FunctionArgument::Enum(enu.clone()),
            proto::function_argument::ArgType::Type(typ) => {
                FunctionArgument::Type(self.decode_type(typ))
            }
            proto::function_argument::ArgType::Value(expr) => {
                FunctionArgument::Value(self.decode_expression(expr))
            }
        }
    }

    fn decode_type(&self, t: &proto::Type) -> Type {
        let kind = t.kind.as_ref().expect("kind must be set");
        match kind {
            // Kind::Bool(_) => {}
            // Kind::I8(_) => {}
            // Kind::I16(_) => {}
            substrait::proto::r#type::Kind::I32(v) => Type::I32 {
                nullable: types::nullability(v.nullability),
            },
            substrait::proto::r#type::Kind::I64(v) => Type::I64 {
                nullable: types::nullability(v.nullability),
            },
            // Kind::Fp32(_) => {}
            substrait::proto::r#type::Kind::Fp64(v) => Type::FP64 {
                nullable: types::nullability(v.nullability),
            },
            substrait::proto::r#type::Kind::String(v) => Type::String {
                nullable: types::nullability(v.nullability),
            },
            // Kind::Binary(_) => {}
            // Kind::Timestamp(_) => {}
            // Kind::Date(_) => {}
            // Kind::Time(_) => {}
            // Kind::IntervalYear(_) => {}
            // Kind::IntervalDay(_) => {}
            // Kind::TimestampTz(_) => {}
            // Kind::Uuid(_) => {}
            // Kind::FixedChar(_) => {}
            // Kind::Varchar(_) => {}
            // Kind::FixedBinary(_) => {}
            // Kind::Decimal(_) => {}
            // Kind::Struct(_) => {}
            // Kind::List(_) => {}
            // Kind::Map(_) => {}
            // Kind::UserDefined(_) => {}
            // Kind::UserDefinedTypeReference(_) => {}
            _ => panic!("cannot handle type: {:?}", kind),
        }
    }

    fn decode_field_reference(&self, fr: &proto::expression::FieldReference) -> FieldReference {
        let root_type = fr.root_type.as_ref().expect("root_type must be present");
        match root_type {
            // proto::expression::field_reference::RootType::Expression(_) => {}
            proto::expression::field_reference::RootType::RootReference(_) => {}
            // proto::expression::field_reference::RootType::OuterReference(_) => {}
            _ => panic!("root_type must be root reference, found: {:?}", root_type),
        }

        match fr.reference_type.as_ref().expect("reference_type is set") {
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

    fn decode_literal(&self, value: &proto::expression::Literal) -> Literal {
        let nullable = value.nullable;
        let literal_type = value
            .literal_type
            .as_ref()
            .expect("literal_type must be set");
        match literal_type {
            proto::expression::literal::LiteralType::Boolean(v) => Literal::Bool(Bool {
                value: *v,
                nullable,
            }),
            // LiteralType::I8(_) => {}
            // LiteralType::I16(_) => {}
            proto::expression::literal::LiteralType::I32(v) => Literal::I32(I32 {
                value: *v,
                nullable,
            }),
            proto::expression::literal::LiteralType::I64(v) => Literal::I64(I64 {
                value: *v,
                nullable,
            }),
            // LiteralType::Fp32(_) => {}
            // LiteralType::Fp64(_) => {}
            // LiteralType::String(_) => {}
            // LiteralType::Binary(_) => {}
            // LiteralType::Timestamp(_) => {}
            // LiteralType::Date(_) => {}
            // LiteralType::Time(_) => {}
            // LiteralType::IntervalYearToMonth(_) => {}
            // LiteralType::IntervalDayToSecond(_) => {}
            // LiteralType::FixedChar(_) => {}
            // LiteralType::VarChar(_) => {}
            // LiteralType::FixedBinary(_) => {}
            // LiteralType::Decimal(_) => {}
            // LiteralType::Struct(_) => {}
            // LiteralType::Map(_) => {}
            // LiteralType::TimestampTz(_) => {}
            // LiteralType::Uuid(_) => {}
            // LiteralType::Null(_) => {}
            // LiteralType::List(_) => {}
            // LiteralType::EmptyList(_) => {}
            // LiteralType::EmptyMap(_) => {}
            // LiteralType::UserDefined(_) => {}
            _ => panic!("cannot handle literal_type: {:?}", literal_type),
        }
    }

    fn gather_extensions(&mut self, p: &proto::Plan) {
        p.extension_uris.iter().for_each(|extension_uri| {
            let anchor = ExtensionAnchor(extension_uri.extension_uri_anchor);
            let uri = URI(extension_uri.uri.clone());
            if self.extension_uri_map.contains_key(&anchor) {
                panic!("anchor {:?} is defined multiple times", anchor)
            }
            self.extension_uri_map.insert(anchor, uri);
        });

        p.extensions.iter().for_each(|extension| {
            match extension
                .mapping_type
                .as_ref()
                .expect("mapping type must be set")
            {
                MappingType::ExtensionType(_) => {
                    panic!("cannot handle type extensions")
                }
                MappingType::ExtensionTypeVariation(_) => {
                    panic!("cannot handle type variation extensions")
                }
                MappingType::ExtensionFunction(ef) => {
                    self.gather_function_mapping(&ef);
                }
            }
        })
    }

    fn gather_function_mapping(&mut self, ef: &ExtensionFunction) {
        let extension = self
            .extension_uri_map
            .get(&ExtensionAnchor(ef.extension_uri_reference))
            .expect("missing extension mapping")
            .clone();
        let function_anchor = FunctionAnchor(ef.function_anchor);
        let signature = FunctionSignature(ef.name.clone());
        if self.function_map.contains_key(&function_anchor) {
            panic!("function anchor {:?} is defined multiple times", signature)
        }

        let function = Function {
            signature,
            extension,
        };
        self.function_map.insert(function_anchor, function);
    }
}
