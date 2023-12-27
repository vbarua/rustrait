use substrait::proto;

#[derive(Debug)]
pub enum Literal {
    Bool(Bool),
    I32(I32),
    I64(I64),
}

impl From<&proto::expression::Literal> for Literal {
    fn from(value: &proto::expression::Literal) -> Self {
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
}

#[derive(Debug)]
pub struct Bool {
    pub value: bool,
    pub nullable: bool,
}

#[derive(Debug)]
pub struct I32 {
    pub value: i32,
    pub nullable: bool,
}

#[derive(Debug)]
pub struct I64 {
    pub value: i64,
    pub nullable: bool,
}
