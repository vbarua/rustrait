#[derive(Debug)]
pub enum Literal {
    Bool(Bool),
    I32(I32),
    I64(I64),
}

macro_rules! literal_struct {
    ($literal_name: ident, $literal_type: tt) => {
        #[derive(Debug)]
        pub struct $literal_name {
            pub value: $literal_type,
            pub nullable: bool,
        }
    };
}

literal_struct![Bool, bool];
literal_struct![I32, i32];
literal_struct![I64, i64];
