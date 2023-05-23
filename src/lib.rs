//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use pgrx::{prelude::*, spi::SpiClient};

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

fn sea_vals_to_args(values: sea_query::Values) -> Vec<(PgOid, Option<pg_sys::Datum>)> {
    values
        .into_iter()
        .map(|v| match v {
            sea_query::Value::Bool(Some(b)) => (PgBuiltInOids::BOOLOID.oid(), b.into_datum()),
            sea_query::Value::TinyInt(Some(i)) => (PgBuiltInOids::INT2OID.oid(), i.into_datum()),
            sea_query::Value::SmallInt(Some(i)) => (PgBuiltInOids::INT2OID.oid(), i.into_datum()),
            sea_query::Value::Int(Some(i)) => (PgBuiltInOids::INT4OID.oid(), i.into_datum()),
            sea_query::Value::BigInt(Some(i)) => (PgBuiltInOids::INT8OID.oid(), i.into_datum()),
            sea_query::Value::Float(Some(f)) => (PgBuiltInOids::FLOAT4OID.oid(), f.into_datum()),
            sea_query::Value::Double(Some(d)) => (PgBuiltInOids::FLOAT8OID.oid(), d.into_datum()),
            sea_query::Value::String(Some(s)) => (PgBuiltInOids::TEXTOID.oid(), s.into_datum()),
            sea_query::Value::Char(Some(c)) => (PgBuiltInOids::CHAROID.oid(), c.into_datum()),
            sea_query::Value::Bytes(Some(b)) => (PgBuiltInOids::BYTEAOID.oid(), b.into_datum()),
            sea_query::Value::Uuid(Some(u)) => {
                let datum = pgrx::Uuid::from_bytes(u.into_bytes()).into_datum();
                (PgBuiltInOids::UUIDOID.oid(), datum)
            }
            _ => todo!(),
        })
        .collect()
}

trait SeaSelect {
    fn sea_select(
        &self,
        select: &mut sea_query::SelectStatement,
    ) -> Result<spi::SpiTupleTable, spi::Error>;
}

impl SeaSelect for SpiClient<'_> {
    fn sea_select(
        &self,
        select: &mut sea_query::SelectStatement,
    ) -> Result<spi::SpiTupleTable, spi::Error> {
        let (query, values) = select.build(sea_query::PostgresQueryBuilder);
        self.select(&query, None, Some(sea_vals_to_args(values)))
    }
}

trait SpiSelect {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error>;
    fn run(&self) -> Result<(), pgrx::spi::Error>;
}

impl SpiSelect for sea_query::SelectStatement {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::get_one_with_args(&query, sea_vals_to_args(values))
        } else {
            Spi::get_one(&query)
        }
    }

    fn run(&self) -> Result<(), pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::run_with_args(&query, Some(sea_vals_to_args(values)))
        } else {
            Spi::run(&query)
        }
    }
}

#[derive(sea_query::Iden)]
enum Schema {
    Table,
    Id,
    Name,
}

#[derive(sea_query::Iden)]
enum SchemaField {
    Table,
    Id,
    SchemaId,
    Path,
    FunType,
    Desc,
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
