use pgrx::prelude::*;

::pgrx::pg_module_magic!();

extension_sql_file!("../sql/regression_pr1817--0.0.0.sql", name="testing");

#[pg_extern]
fn hello_regression_pr1817() -> &'static str {
    "Hello, regression_pr1817"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_regression_pr1817() {
        assert_eq!("Hello, regression_pr1817", crate::hello_regression_pr1817());
    }

    #[pg_test]
    fn test_generated_schema() {
        // run the "hellp world" function through SPI. This will fail if the generated schema (containing the function)
        // was overwritten
        Spi::run("SELECT hello_regression_pr1817();").expect("unable to run function from generated schema");
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
