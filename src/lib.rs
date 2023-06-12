//! PostgreSQL extension for building dynamic rules and tracking data for tabletop role-playing games.

use models::{Character, CharacterTrait, FunField, NumericRule, Trait};
use pgrx::prelude::*;
use sea_ext::SeaGetOneExt;
use sea_query::{Expr, Query};

mod migrations;
mod models;
mod sea_ext;
mod seed;
mod spi_heap_tuple_data_ext;
mod uuid7;

pgrx::pg_module_magic!();

#[pg_extern]
fn character_field(
    character_id: pgrx::Uuid,
    field_id: pgrx::Uuid,
) -> Result<Option<pgrx::AnyNumeric>, spi::Error> {
    let character_id = uuid::Uuid::from_bytes(*character_id.as_bytes());
    let field_id = uuid::Uuid::from_bytes(*field_id.as_bytes());
    Query::select()
        .from(FunField::Table)
        .inner_join(
            NumericRule::Table,
            Expr::col((FunField::Table, FunField::Id))
                .equals((NumericRule::Table, NumericRule::FieldId)),
        )
        .inner_join(
            Trait::Table,
            Expr::col((NumericRule::Table, NumericRule::TraitId)).equals((Trait::Table, Trait::Id)),
        )
        .inner_join(
            CharacterTrait::Table,
            Expr::col((Trait::Table, Trait::Id))
                .equals((CharacterTrait::Table, CharacterTrait::TraitId)),
        )
        .inner_join(
            Character::Table,
            Expr::col((CharacterTrait::Table, CharacterTrait::CharacterId))
                .equals((Character::Table, Character::Id)),
        )
        .and_where(Expr::col(FunField::Id).eq(field_id))
        .and_where(Expr::col(Character::Id).eq(character_id))
        .group_by_col((FunField::Table, FunField::Id))
        .expr(Expr::col((NumericRule::Table, NumericRule::Value)).sum())
        .get_one()
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
