// tests module for testing the application, this module is only included in the final build when the `test` feature is enabled
#[cfg(test)]
#[allow(clippy::module_inception)]
mod action_test;

// utils module only used for testing and not included in the final build, this module is only included in the final build when the `test` feature is enabled
#[cfg(test)]
mod database_test;
#[cfg(test)]
mod standard_test;
#[cfg(test)]
mod struct_test;
#[cfg(test)]
mod utils;
