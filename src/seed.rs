use crate::{
    models::{Character, CharacterTrait, FunField, FunSchema, NumericRule, Trait},
    sea_ext::SeaRunExt,
};
use pgrx::prelude::*;

#[pg_extern]
pub fn fun_seed() -> Result<(), spi::Error> {
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
        .columns([FunField::Id, FunField::SchemaId, FunField::Name])
        .values_panic([STR_FIELD_ID.into(), SCHEMA_ID.into(), "strength".into()])
        .values_panic([DEX_FIELD_ID.into(), SCHEMA_ID.into(), "dexterity".into()])
        .values_panic([CON_FIELD_ID.into(), SCHEMA_ID.into(), "constitution".into()])
        .values_panic([INT_FIELD_ID.into(), SCHEMA_ID.into(), "intelligence".into()])
        .values_panic([WIS_FIELD_ID.into(), SCHEMA_ID.into(), "wisdom".into()])
        .values_panic([CHA_FIELD_ID.into(), SCHEMA_ID.into(), "charisma".into()])
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
        .into_table(NumericRule::Table)
        .columns([
            NumericRule::TraitId,
            NumericRule::FieldId,
            NumericRule::Value,
        ])
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
