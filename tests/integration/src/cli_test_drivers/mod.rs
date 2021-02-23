pub mod setup;
pub mod test_runner;
pub mod environment;
pub mod tls;

mod smoke;
mod stress;

pub use driver::*;
pub use common::*;

mod common {

    use async_trait::async_trait;

    #[async_trait]
    pub trait TestDriver {
        /// run tester
        async fn run(&self);
    }
}

/// select runner based on option
mod driver {

    use crate::TestOption;
    use crate::cli_test_drivers::smoke::SmokeTestRunner;

    use super::TestDriver;

    pub fn create_test_driver(option: TestOption) -> Box<dyn TestDriver> {
        Box::new(SmokeTestRunner::new(option))
    }
}
