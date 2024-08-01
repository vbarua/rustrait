use crate::plans::expressions::{Function, FunctionArgument};
use crate::types::Type;

#[derive(Debug)]
pub struct ScalarFunctionInvocation {
    pub function: Function,
    pub args: Vec<FunctionArgument>,
    pub output_type: Type,
}
