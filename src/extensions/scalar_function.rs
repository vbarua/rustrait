use substrait::text::simple_extensions;

use crate::types::Type;

#[derive(Debug)]
pub struct ScalarFunction {
    pub name: String,
    pub variants: Vec<ScalarFunctionVariant>,
}

impl From<simple_extensions::ScalarFunction> for ScalarFunction {
    fn from(sf: simple_extensions::ScalarFunction) -> Self {
        return ScalarFunction {
            name: sf.name,
            variants: sf
                .impls
                .into_iter()
                .map(|sfii| ScalarFunctionVariant::from(sfii))
                .collect(),
        };
    }
}

#[derive(Debug)]
pub struct ScalarFunctionVariant {
    pub arguments: Vec<String>,
    // pub options: Vec<Option>,
    // pub nullability,
    // pub variadicBehaviour,
    // TODO: moar
    pub return_type: Type,
}

impl From<simple_extensions::ScalarFunctionImplsItem> for ScalarFunctionVariant {
    fn from(sfii: simple_extensions::ScalarFunctionImplsItem) -> Self {
        let return_type = match sfii.return_.0 {
            simple_extensions::Type::Variant0(str) => Type::from(str.as_str()),
            simple_extensions::Type::Variant1(_) => panic!("what even is this?!?!?"),
        };
        ScalarFunctionVariant {
            arguments: vec![],
            return_type: return_type,
        }
    }
}
