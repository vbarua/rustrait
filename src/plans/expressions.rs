use substrait::proto;

use crate::plans::expressions::field_reference::FieldReference;
use crate::plans::expressions::literal::Literal;

mod field_reference;
mod literal;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    FieldReference(FieldReference),
}

impl From<&proto::Expression> for Expression {
    fn from(expr: &proto::Expression) -> Self {
        match expr.rex_type.as_ref().expect("rex_type is set") {
            proto::expression::RexType::Literal(literal) => {
                Expression::Literal(Literal::from(literal))
            }
            proto::expression::RexType::Selection(field_reference) => {
                Expression::FieldReference(FieldReference::from(field_reference.as_ref()))
            }
            // RexType::ScalarFunction(_) => {}
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
}
