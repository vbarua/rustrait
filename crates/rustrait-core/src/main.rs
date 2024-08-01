use crate::decoder::decode_prost_plan;
use substrait::proto;
use substrait::text::simple_extensions::SimpleExtensions;

pub mod decoder;
pub mod extensions;
pub mod plans;
pub mod types;

fn main() {
    let functions_arithmetic = r#"
    scalar_functions:
      -
        name: "add"
        description: "Add two values."
        impls:
          - args:
             - name: x
               value: i64
             - name: y
               value: i64
            options:
              overflow:
                values: [ SILENT, SATURATE, ERROR ]
            return: i64
    "#;

    let plan_string = include_str!("../simple-select.substrait");
    let proto_plan = serde_json::from_str::<proto::Plan>(plan_string).expect("success!?!?");

    let simple_extension = serde_yaml::from_str::<SimpleExtensions>(functions_arithmetic);
    let extensions = extensions::Extensions::from(simple_extension.expect("boom"));
    dbg!(extensions);

    let plan = decode_prost_plan(&proto_plan);
    dbg!(plan);
}
