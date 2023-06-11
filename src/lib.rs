//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use crate::models::{Character, CharacterTrait, FunField, Rule, Trait};
use models::FunSchema;
use pgrx::prelude::*;
use sea_ext::SeaRunExt;
use sea_query::Expr;

mod fun_type;
mod migrations;
mod models;
mod sea_ext;
mod sea_select;
mod spi_heap_tuple_data_ext;
mod uuid7;

pgrx::pg_module_magic!();

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
            sea_query::Value::Uuid(Some(u)) => (
                PgBuiltInOids::UUIDOID.oid(),
                pgrx::Uuid::from_bytes(u.into_bytes()).into_datum(),
            ),
            _ => todo!(),
        })
        .collect()
}

#[pg_extern]
fn fun_seed() -> Result<(), spi::Error> {
    const SCHEMA_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x69, 0xf0, 0x3e, 0xb0,
        0x7d,
    ]);
    const STR_FIELD_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6a, 0x47, 0x5c, 0x2c,
        0x5f,
    ]);
    const DEX_FIELD_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6b, 0x67, 0x43, 0x0f,
        0x80,
    ]);
    const CON_FIELD_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6c, 0x33, 0xdc, 0x00,
        0x78,
    ]);
    const INT_FIELD_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6d, 0x76, 0x65, 0x27,
        0xd9,
    ]);
    const WIS_FIELD_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6e, 0x79, 0xad, 0x79,
        0x5c,
    ]);
    const CHA_FIELD_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6f, 0xbc, 0x7f, 0xab,
        0xfc,
    ]);
    const CHAR1_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x70, 0x22, 0xe9, 0xd7,
        0x5d,
    ]);
    const CHAR2_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x71, 0xf4, 0x62, 0x47,
        0x47,
    ]);
    const BASE_TRAIT_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x72, 0xcd, 0x3b, 0xd9,
        0xdc,
    ]);
    const DWARF_TRAIT_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x73, 0x25, 0x66, 0x65,
        0x92,
    ]);
    const ELF_TRAIT_ID: uuid::Uuid = uuid::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x74, 0xfa, 0x9e, 0x34,
        0x8d,
    ]);

    sea_query::Query::insert()
        .into_table(FunSchema::Table)
        .columns([FunSchema::Id, FunSchema::Name])
        .values_panic([SCHEMA_ID.into(), "foo".into()])
        .run()?;

    sea_query::Query::insert()
        .into_table(FunField::Table)
        .columns([
            FunField::Id,
            FunField::SchemaId,
            FunField::FunType,
            FunField::Field,
        ])
        .values_panic([
            STR_FIELD_ID.into(),
            SCHEMA_ID.into(),
            Expr::cust("'Int4'"),
            "strength".into(),
        ])
        .values_panic([
            DEX_FIELD_ID.into(),
            SCHEMA_ID.into(),
            Expr::cust("'Int4'"),
            "dexterity".into(),
        ])
        .values_panic([
            CON_FIELD_ID.into(),
            SCHEMA_ID.into(),
            Expr::cust("'Int4'"),
            "constitution".into(),
        ])
        .values_panic([
            INT_FIELD_ID.into(),
            SCHEMA_ID.into(),
            Expr::cust("'Int4'"),
            "intelligence".into(),
        ])
        .values_panic([
            WIS_FIELD_ID.into(),
            SCHEMA_ID.into(),
            Expr::cust("'Int4'"),
            "wisdom".into(),
        ])
        .values_panic([
            CHA_FIELD_ID.into(),
            SCHEMA_ID.into(),
            Expr::cust("'Int4'"),
            "charisma".into(),
        ])
        .run()?;

    sea_query::Query::insert()
        .into_table(Character::Table)
        .columns([Character::Id, Character::Name])
        .values_panic([CHAR1_ID.into(), "Braugnor Quickcleaver".into()])
        .values_panic([CHAR2_ID.into(), "Jaudenn Runecleaver".into()])
        .run()?;

    sea_query::Query::insert()
        .into_table(Trait::Table)
        .columns([Trait::Id, Trait::Name])
        .values_panic([BASE_TRAIT_ID.into(), "Base".into()])
        .values_panic([DWARF_TRAIT_ID.into(), "Dwarf".into()])
        .values_panic([ELF_TRAIT_ID.into(), "Elf".into()])
        .run()?;

    sea_query::Query::insert()
        .into_table(Rule::Table)
        .columns([Rule::TraitId, Rule::FieldId, Rule::Value])
        .values_panic([BASE_TRAIT_ID.into(), STR_FIELD_ID.into(), 8.into()])
        .values_panic([BASE_TRAIT_ID.into(), DEX_FIELD_ID.into(), 8.into()])
        .values_panic([BASE_TRAIT_ID.into(), CON_FIELD_ID.into(), 8.into()])
        .values_panic([BASE_TRAIT_ID.into(), INT_FIELD_ID.into(), 8.into()])
        .values_panic([BASE_TRAIT_ID.into(), WIS_FIELD_ID.into(), 8.into()])
        .values_panic([BASE_TRAIT_ID.into(), CHA_FIELD_ID.into(), 8.into()])
        .values_panic([DWARF_TRAIT_ID.into(), STR_FIELD_ID.into(), 2.into()])
        .values_panic([ELF_TRAIT_ID.into(), DEX_FIELD_ID.into(), 2.into()])
        .run()?;

    sea_query::Query::insert()
        .into_table(CharacterTrait::Table)
        .columns([CharacterTrait::CharacterId, CharacterTrait::TraitId])
        .values_panic([CHAR1_ID.into(), BASE_TRAIT_ID.into()])
        .values_panic([CHAR2_ID.into(), BASE_TRAIT_ID.into()])
        .values_panic([CHAR1_ID.into(), DWARF_TRAIT_ID.into()])
        .values_panic([CHAR2_ID.into(), ELF_TRAIT_ID.into()])
        .run()?;

    Ok(())
}

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
