use crate::{
    into_pgrx_arg::IntoPgrxArg,
    models::{Character, CharacterTrait, Field, NumericRule, Schema, Trait},
};
use pgrx::prelude::*;

#[pg_extern]
pub fn fun_seed() -> Result<(), spi::Error> {
    let schema_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x69, 0xf0, 0x3e, 0xb0,
        0x7d,
    ])
    .into_arg();
    let str_field_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6a, 0x47, 0x5c, 0x2c,
        0x5f,
    ])
    .into_arg();
    let dex_field_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6b, 0x67, 0x43, 0x0f,
        0x80,
    ])
    .into_arg();
    let con_field_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6c, 0x33, 0xdc, 0x00,
        0x78,
    ])
    .into_arg();
    let int_field_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6d, 0x76, 0x65, 0x27,
        0xd9,
    ])
    .into_arg();
    let wis_field_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6e, 0x79, 0xad, 0x79,
        0x5c,
    ])
    .into_arg();
    let cha_field_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x6f, 0xbc, 0x7f, 0xab,
        0xfc,
    ])
    .into_arg();
    let char1_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x70, 0x22, 0xe9, 0xd7,
        0x5d,
    ])
    .into_arg();
    let char2_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x71, 0xf4, 0x62, 0x47,
        0x47,
    ])
    .into_arg();
    let base_trait_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x72, 0xcd, 0x3b, 0xd9,
        0xdc,
    ])
    .into_arg();
    let dwarf_trait_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x73, 0x25, 0x66, 0x65,
        0x92,
    ])
    .into_arg();
    let elf_trait_id = pgrx::Uuid::from_bytes([
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x74, 0xfa, 0x9e, 0x34,
        0x8d,
    ])
    .into_arg();

    Spi::run_with_args(
        &format!("INSERT INTO {} VALUES ($1, 'foo')", Schema::Table),
        Some(vec![schema_id]),
    )?;

    Spi::run_with_args(
        &format!(
            r#"
        INSERT INTO {field} ({field_id}, {field_schema_id}, {field_name}) VALUES
            ($2, $1, 'strength'),
            ($3, $1, 'dexterity'),
            ($4, $1, 'constitution'),
            ($5, $1, 'intelligence'),
            ($6, $1, 'wisdom'),
            ($7, $1, 'charisma');
        "#,
            field = Field::Table,
            field_id = Field::Id,
            field_schema_id = Field::SchemaId,
            field_name = Field::Name
        ),
        Some(vec![
            schema_id,
            str_field_id,
            dex_field_id,
            con_field_id,
            int_field_id,
            wis_field_id,
            cha_field_id,
        ]),
    )?;

    Spi::run_with_args(
        &format!(
            r#"
        INSERT INTO {} VALUES
            ($1, 'Braugnor Quickcleaver'),
            ($2, 'Jaudenn Runecleaver');
        "#,
            Character::Table
        ),
        Some(vec![char1_id, char2_id]),
    )?;

    Spi::run_with_args(
        &format!(
            r#"
        INSERT INTO {} VALUES
            ($1, 'base'),
            ($2, 'dwarf'),
            ($3, 'elf');
        "#,
            Trait::Table
        ),
        Some(vec![base_trait_id, dwarf_trait_id, elf_trait_id]),
    )?;

    Spi::run_with_args(
        &format!(
            r#"
        INSERT INTO {} VALUES
            ($1, $7, 8),
            ($2, $7, 8),
            ($3, $7, 8),
            ($4, $7, 8),
            ($5, $7, 8),
            ($6, $7, 8),
            ($1, $8, 2),
            ($2, $9, 2);
        "#,
            NumericRule::Table
        ),
        Some(vec![
            str_field_id,
            dex_field_id,
            con_field_id,
            int_field_id,
            wis_field_id,
            cha_field_id,
            base_trait_id,
            dwarf_trait_id,
            elf_trait_id,
        ]),
    )?;

    Spi::run_with_args(
        &format!(
            r#"
        INSERT INTO {} VALUES
            ($1, $3),
            ($2, $3),
            ($1, $4),
            ($2, $5);
        "#,
            CharacterTrait::Table
        ),
        Some(vec![
            char1_id,
            char2_id,
            base_trait_id,
            dwarf_trait_id,
            elf_trait_id,
        ]),
    )?;

    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_fun_seed() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT fun_seed();")?;
        Ok(())
    }
}
