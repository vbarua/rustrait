use substrait::proto;
use substrait::text::simple_extensions::SimpleExtensions;

use prost::Message;

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

    let plan_bytes = include_bytes!("../simpleselect.bin");
    let proto_plan = proto::Plan::decode(&plan_bytes[..]).expect("success!?!?");

    let simple_extension = serde_yaml::from_str::<SimpleExtensions>(functions_arithmetic);
    let extensions = extensions::Extensions::from(simple_extension.expect("boom"));
    dbg!(extensions);

    let plan = plans::Plan::from(&proto_plan);
    dbg!(plan);
}
