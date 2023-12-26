use substrait::proto;

#[derive(Debug)]
pub enum Type {
    I64 { nullable: bool },
    FP64 { nullable: bool },
    String { nullable: bool },
    Struct { nullable: bool, types: Vec<Type> },
}

#[derive(Debug)]
pub struct NamedStruct {
    // TODO: should these be combained into a vec of tuples?
    //    Would that even be valid given that these could have different lengths?
    pub names: Vec<String>,
    pub types: Vec<Type>,
}

impl From<&proto::NamedStruct> for NamedStruct {
    fn from(ns: &proto::NamedStruct) -> Self {
        NamedStruct {
            names: ns.names.clone(),
            types: ns
                .r#struct
                .as_ref()
                .expect("types should be present")
                .types
                .iter()
                .map(|t| Type::from(t))
                .collect(),
        }
    }
}

const NULLABILITY_UNSPECIFIED: i32 = 0;
const NULLABILITY_NULLABLE: i32 = 1;
const NULLABILITY_REQUIRED: i32 = 2;

fn nullability(n: i32) -> bool {
    match n {
        NULLABILITY_UNSPECIFIED => panic!("nullability must be specified"),
        NULLABILITY_NULLABLE => return false,
        NULLABILITY_REQUIRED => return true,
        _ => panic!("you are using a secret 4th nullability {}. please don't", n),
    }
}

impl From<&proto::Type> for Type {
    fn from(t: &proto::Type) -> Self {
        let kind = t.kind.as_ref().expect("kind must be set");
        match kind {
            // Kind::Bool(_) => {}
            // Kind::I8(_) => {}
            // Kind::I16(_) => {}
            // Kind::I32(_) => {}
            substrait::proto::r#type::Kind::I64(i64) => Type::I64 {
                nullable: nullability(i64.nullability),
            },
            // Kind::Fp32(_) => {}
            substrait::proto::r#type::Kind::Fp64(fp64) => Type::FP64 {
                nullable: nullability(fp64.nullability),
            },
            substrait::proto::r#type::Kind::String(s) => Type::String {
                nullable: nullability(s.nullability),
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
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "i64" => Type::I64 { nullable: false },
            "fp64" => Type::FP64 { nullable: false },
            "str" => Type::String { nullable: false },
            _ => panic!("cannot produce type from type string: {}", s),
        }
    }
}

impl From<substrait::text::simple_extensions::Type> for Type {
    fn from(t: substrait::text::simple_extensions::Type) -> Self {
        match t {
            substrait::text::simple_extensions::Type::Variant0(str) => Type::from(str.as_str()),
            // TODO: What even is this?
            substrait::text::simple_extensions::Type::Variant1(_) => panic!(),
        }
    }
}
