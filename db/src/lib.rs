//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use pgrx::prelude::*;

mod base58;
mod into_pgrx_arg;
mod migrations;
mod seed;
mod slugify;
mod uuid7;

pgrx::pg_module_magic!();

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    /// Perform one-off initialization when the pg_test framework starts
    pub fn setup(_options: Vec<&str>) {}

    /// Return any postgresql.conf settings that are required for your tests
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
