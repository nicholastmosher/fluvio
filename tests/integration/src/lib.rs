mod tests;

pub mod cli_test_drivers;

pub use fluvio_test_options::*;
pub use cli_test_drivers::tls::*;

const VERSION: &str = include_str!("../../../VERSION");
