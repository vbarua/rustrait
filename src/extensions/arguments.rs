use substrait::text::simple_extensions;

use crate::types::Type;

pub enum Argument {
    Value {
        name: Option<String>,
        value: Type,
        is_constant: bool,
    },
}

impl From<simple_extensions::ArgumentsItem> for Argument {
    fn from(ai: simple_extensions::ArgumentsItem) -> Self {
        match ai {
            // enum
            simple_extensions::ArgumentsItem::Variant0 { .. } => {
                panic!("cannot handle enum argument")
            }
            // value
            simple_extensions::ArgumentsItem::Variant1 {
                constant,
                name,
                value,
                ..
            } => Argument::Value {
                name: name,
                value: Type::from(value),
                is_constant: constant.unwrap_or(false),
            },
            // type
            simple_extensions::ArgumentsItem::Variant2 { .. } => {
                panic!("cannot handle type argument")
            }
        }
    }
}
