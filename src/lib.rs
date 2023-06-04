//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use crate::models::{Char, CharTrait, Effect, SchemaField, Trait};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use models::Schema;
use pgrx::prelude::*;
use rand::rngs::ThreadRng;
use sea_ext::SeaRunExt;
use sea_query::Expr;

mod char_aggr_sync;
mod fun_type;
mod migrations;
mod models;
mod refresh_char_aggr;
mod refresh_char_aggr_trigger;
mod sea_ext;
mod sea_select;
mod spi_heap_tuple_data_ext;

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
fn uuid7_to_time(uuid: pgrx::Uuid) -> Result<pgrx::Timestamp, &'static str> {
    if uuid[6] >> 4 == 7 {
        let ts = &uuid[0..6];
        let ms = (ts[0] as i64) << 40
            | (ts[1] as i64) << 32
            | (ts[2] as i64) << 24
            | (ts[3] as i64) << 16
            | (ts[4] as i64) << 8
            | (ts[5] as i64);

        if let Some(date) = NaiveDateTime::from_timestamp_millis(ms) {
            let time = date.time();

            Ok(pgrx::Timestamp::new_unchecked(
                date.year() as isize,
                date.month() as u8,
                date.day() as u8,
                time.hour() as u8,
                time.minute() as u8,
                f64::from(time.nanosecond()) / 1_000_000_000.0,
            ))
        } else {
            Err("Cannot convert timestamp to date.")
        }
    } else {
        Err("Wrong UUID version.")
    }
}

#[pg_extern]
fn gen_rand_uuid7() -> Result<pgrx::Uuid, String> {
    pgrx::Uuid::from_slice(uuid7::uuid7().as_bytes())
}

#[pg_extern]
fn gen_uuid7(ts: pgrx::Timestamp) -> Result<pgrx::Uuid, String> {
    let (h, m, s, ms) = ts.to_hms_micro();
    if let Some(datetime) =
        NaiveDate::from_ymd_opt(ts.year(), u32::from(ts.month()), u32::from(ts.day()))
            .and_then(|d| d.and_hms_micro_opt(u32::from(h), u32::from(m), u32::from(s), ms))
    {
        pgrx::Uuid::from_slice(
            uuid7::V7Generator::new(ThreadRng::default())
                .generate_or_reset_core(datetime.timestamp_millis() as u64, 10_000)
                .as_bytes(),
        )
    } else {
        Err("Cannot convert timestamp to date.".to_string())
    }
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
        .into_table(Schema::Table)
        .columns([Schema::Id, Schema::Name])
        .values_panic([SCHEMA_ID.into(), "foo".into()])
        .run()?;

    sea_query::Query::insert()
        .into_table(SchemaField::Table)
        .columns([
            SchemaField::Id,
            SchemaField::SchemaId,
            SchemaField::FunType,
            SchemaField::Path,
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
        .into_table(Char::Table)
        .columns([Char::Id, Char::Name])
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
        .into_table(Effect::Table)
        .columns([Effect::TraitId, Effect::SchemaFieldId, Effect::Val])
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
        .into_table(CharTrait::Table)
        .columns([CharTrait::CharId, CharTrait::TraitId])
        .values_panic([CHAR1_ID.into(), BASE_TRAIT_ID.into()])
        .values_panic([CHAR2_ID.into(), BASE_TRAIT_ID.into()])
        .values_panic([CHAR1_ID.into(), DWARF_TRAIT_ID.into()])
        .values_panic([CHAR2_ID.into(), ELF_TRAIT_ID.into()])
        .run()?;

    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    const UUID: pgrx::UuidBytes = [
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x69, 0xf0, 0x3e, 0xb0,
        0x7d,
    ];

    #[pg_test]
    fn test_uuid7_to_time() {
        assert_eq!(
            Ok(Some(pgrx::Timestamp::new_unchecked(
                2023, 5, 29, 10, 36, 0.724
            ))),
            Spi::get_one_with_args::<pgrx::Timestamp>(
                "SELECT uuid7_to_time($1);",
                vec![(
                    PgBuiltInOids::UUIDOID.oid(),
                    pgrx::Uuid::from_bytes(UUID).into_datum(),
                )],
            )
        );
    }

    #[pg_test]
    fn test_gen_uuid7() {
        let ts = pgrx::Timestamp::new_unchecked(2000, 1, 1, 0, 0, 0.0);
        let uuid = Spi::get_one_with_args::<pgrx::Uuid>(
            "SELECT gen_uuid7($1);",
            vec![(PgBuiltInOids::TIMESTAMPOID.oid(), ts.into_datum())],
        )
        .ok()
        .flatten()
        .unwrap();
        assert_eq!(
            Ok(Some(ts)),
            Spi::get_one_with_args::<pgrx::Timestamp>(
                "SELECT uuid7_to_time($1);",
                vec![(PgBuiltInOids::UUIDOID.oid(), uuid.into_datum(),)],
            )
        );
    }
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
