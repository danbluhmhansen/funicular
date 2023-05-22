//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

mod char_aggr_sync;
mod refresh_char_aggr;
mod refresh_char_aggr_trigger;

pgrx::pg_module_magic!();

#[derive(Debug, Serialize, Deserialize, PostgresEnum, PartialEq)]
pub enum FunType {
    Int2,
    Int4,
    Int8,
    Uuid,
    Text,
    Bool,
    Float4,
    Float8,
    Numeric,
    Date,
    Time,
    Timez,
    Timestamp,
    Timestampz,
    Json,
    Jsonb,
}

impl FunType {
    pub fn to_pg_type(&self) -> &'static str {
        match self {
            FunType::Int2 => "smallint",
            FunType::Int4 => "integer",
            FunType::Int8 => "bigint",
            FunType::Uuid => "uuid",
            FunType::Text => "text",
            FunType::Bool => "boolean",
            FunType::Float4 => "real",
            FunType::Float8 => "double precision",
            FunType::Numeric => "numeric",
            FunType::Date => "date",
            FunType::Time => "time",
            FunType::Timez => "timez",
            FunType::Timestamp => "timestamp",
            FunType::Timestampz => "timestampz",
            FunType::Json => "json",
            FunType::Jsonb => "jsonb",
        }
    }

    pub fn to_pg_oid(&self) -> PgBuiltInOids {
        match self {
            FunType::Int2 => PgBuiltInOids::INT2OID,
            FunType::Int4 => PgBuiltInOids::INT4OID,
            FunType::Int8 => PgBuiltInOids::INT8OID,
            FunType::Uuid => PgBuiltInOids::UUIDOID,
            FunType::Text => PgBuiltInOids::TEXTOID,
            FunType::Bool => PgBuiltInOids::BOOLOID,
            FunType::Float4 => PgBuiltInOids::FLOAT4OID,
            FunType::Float8 => PgBuiltInOids::FLOAT8OID,
            FunType::Numeric => PgBuiltInOids::NUMERICOID,
            FunType::Date => PgBuiltInOids::DATEOID,
            FunType::Time => PgBuiltInOids::TIMEOID,
            FunType::Timez => PgBuiltInOids::TIMETZOID,
            FunType::Timestamp => PgBuiltInOids::TIMESTAMPOID,
            FunType::Timestampz => PgBuiltInOids::TIMESTAMPTZOID,
            FunType::Json => PgBuiltInOids::JSONOID,
            FunType::Jsonb => PgBuiltInOids::JSONBOID,
        }
    }
}

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
