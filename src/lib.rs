//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use pgrx::prelude::*;

mod char_aggr_sync;
mod refresh_char_aggr;
mod refresh_char_aggr_trigger;

pgrx::pg_module_magic!();

#[cfg(any(debug_assertions, test))]
extension_sql_file!("../static/up.sql");

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
