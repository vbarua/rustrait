use substrait::text::simple_extensions;

mod arguments;
mod scalar_function;

// TODO:
// * types
// * aggregate functions
// * window functions
#[derive(Debug)]
pub struct Extensions {
    pub scalar_functions: Vec<scalar_function::ScalarFunction>,
}

impl From<simple_extensions::SimpleExtensions> for Extensions {
    fn from(se: simple_extensions::SimpleExtensions) -> Self {
        return Extensions {
            scalar_functions: se
                .scalar_functions
                .into_iter()
                .map(|sf| scalar_function::ScalarFunction::from(sf))
                .collect(),
        };
    }
}
