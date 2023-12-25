#[derive(Debug)]
pub enum Type {
    I64 { nullable: bool },
    FP64 { nullable: bool },
    String { nullable: bool },
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "i64" => Type::I64 { nullable: false },
            "fp64" => Type::FP64 { nullable: false },
            "str" => Type::String { nullable: false },
            _ => panic!("cannot produce type from type string: "),
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
