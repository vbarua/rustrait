use crate::plans::expressions::Expression;
use crate::types::NamedStruct;

pub mod expressions;

#[derive(Debug)]
pub struct Plan {
    pub root: Box<Rel>,
}

// Should this be an enum or should it be done via traits ?!?!?
#[derive(Debug)]
pub enum Rel {
    Read(Read),
    Project(Project),
}

#[derive(Debug)]
pub struct Read {
    pub base_schema: NamedStruct,
}

#[derive(Debug)]
pub struct Project {
    pub input: Box<Rel>,
    pub expressions: Vec<Expression>,
}
