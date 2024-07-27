use crate::plans::expressions::field_reference::FieldReference;
use crate::plans::expressions::literal::Literal;
use crate::plans::expressions::scalar_function::ScalarFunctionInvocation;
use crate::types::Type;

pub mod field_reference;
pub mod literal;
pub mod scalar_function;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    FieldReference(FieldReference),
    ScalarFunction(ScalarFunctionInvocation),
}

#[derive(Debug)]
pub enum FunctionArgument {
    Value(Expression),
    // TODO: Improve Enum Argument modelling
    Enum(String),
    Type(Type),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub extension: URI,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature(pub String);

#[derive(Debug, Clone)]
pub struct URI(pub String);
