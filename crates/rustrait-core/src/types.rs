#[derive(Debug)]
pub enum Type {
    I32 { nullable: bool },
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

const NULLABILITY_UNSPECIFIED: i32 = 0;
const NULLABILITY_NULLABLE: i32 = 1;
const NULLABILITY_REQUIRED: i32 = 2;

pub fn nullability(n: i32) -> bool {
    match n {
        NULLABILITY_UNSPECIFIED => panic!("nullability must be specified"),
        NULLABILITY_NULLABLE => return false,
        NULLABILITY_REQUIRED => return true,
        _ => panic!("you are using a secret 4th nullability {}. please don't", n),
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
