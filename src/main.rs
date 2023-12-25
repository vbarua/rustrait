use substrait::text::simple_extensions::SimpleExtensions;

pub mod extensions;
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

    let simple_extension = serde_yaml::from_str::<SimpleExtensions>(functions_arithmetic);
    let extensions = extensions::Extensions::from(simple_extension.expect("boom"));
    dbg!(extensions);
}
