use crate::{
    migrations::Migration,
    models::{
        Character, CharacterTrait, FunField, FunSchema, NumericRule, TextRule, Trait, _Migration,
    },
    sea_ext::SeaRunExt,
    uuid7::GenRandUuid7,
};
use pgrx::prelude::*;
use sea_query::{
    ColumnDef, Expr, ForeignKey, ForeignKeyAction, Func, Iden, Index, IntoIden, Query, Table,
};
use std::iter::once;

#[derive(Iden)]
struct _230603095553Init;

impl Migration for _230603095553Init {
    fn up() -> Result<(), spi::Error> {
        Table::create()
            .table(FunSchema::Table)
            .col(
                ColumnDef::new(FunSchema::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(
                ColumnDef::new(FunSchema::Name)
                    .text()
                    .not_null()
                    .unique_key()
                    .extra(format!(
                        "CHECK ({} ~ '^[a-z_]*$')",
                        FunSchema::Name.into_iden().to_string()
                    )),
            )
            .run()?;

        Table::create()
            .table(FunField::Table)
            .col(
                ColumnDef::new(FunField::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(ColumnDef::new(FunField::SchemaId).uuid().not_null())
            .col(ColumnDef::new(FunField::FieldId).uuid())
            .col(
                ColumnDef::new(FunField::Name)
                    .text()
                    .not_null()
                    .extra(format!(
                        "CHECK ({} ~ '^[a-z_]*$')",
                        FunField::Name.into_iden().to_string()
                    )),
            )
            .col(ColumnDef::new(FunField::Description).text())
            .foreign_key(
                ForeignKey::create()
                    .from(FunField::Table, FunField::SchemaId)
                    .to(FunSchema::Table, FunSchema::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FunField::Table, FunField::FieldId)
                    .to(FunField::Table, FunField::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Table::create()
            .table(Character::Table)
            .col(
                ColumnDef::new(Character::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(ColumnDef::new(Character::Name).text().not_null())
            .run()?;

        Table::create()
            .table(Trait::Table)
            .col(
                ColumnDef::new(Trait::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(ColumnDef::new(Trait::Name).text().not_null())
            .run()?;

        Table::create()
            .table(NumericRule::Table)
            .col(ColumnDef::new(NumericRule::FieldId).uuid().not_null())
            .col(ColumnDef::new(NumericRule::TraitId).uuid().not_null())
            .col(ColumnDef::new(NumericRule::Value).decimal().not_null())
            .primary_key(
                Index::create()
                    .col(NumericRule::FieldId)
                    .col(NumericRule::TraitId),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NumericRule::Table, NumericRule::FieldId)
                    .to(FunField::Table, FunField::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NumericRule::Table, NumericRule::TraitId)
                    .to(Trait::Table, Trait::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Table::create()
            .table(TextRule::Table)
            .col(ColumnDef::new(TextRule::FieldId).uuid().not_null())
            .col(ColumnDef::new(TextRule::TraitId).uuid().not_null())
            .col(ColumnDef::new(TextRule::Value).text().not_null())
            .primary_key(
                Index::create()
                    .col(TextRule::FieldId)
                    .col(TextRule::TraitId),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TextRule::Table, TextRule::FieldId)
                    .to(FunField::Table, FunField::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TextRule::Table, TextRule::TraitId)
                    .to(Trait::Table, Trait::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Table::create()
            .table(CharacterTrait::Table)
            .col(
                ColumnDef::new(CharacterTrait::CharacterId)
                    .uuid()
                    .not_null(),
            )
            .col(ColumnDef::new(CharacterTrait::TraitId).uuid().not_null())
            .primary_key(
                Index::create()
                    .col(CharacterTrait::CharacterId)
                    .col(CharacterTrait::TraitId),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CharacterTrait::Table, CharacterTrait::CharacterId)
                    .to(Character::Table, Character::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CharacterTrait::Table, CharacterTrait::TraitId)
                    .to(Trait::Table, Trait::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Spi::run(
            r#"
CREATE VIEW character_numeric_field AS
SELECT
	character.id AS character_id,
	fun_field.id AS field_id,
	SUM(numeric_rule.value)
FROM fun_field
JOIN numeric_rule ON numeric_rule.field_id = fun_field.id
JOIN trait ON trait.id = numeric_rule.trait_id
JOIN character_trait ON character_trait.trait_id = trait.id
JOIN character ON character.id = character_trait.character_id
GROUP BY fun_field.id, character.id
ORDER BY character.id;
            "#,
        )?;

        Query::insert()
            .into_table(_Migration::Table)
            .columns([_Migration::Name])
            .values_panic(once(_230603095553Init.to_string().into()))
            .run()?;

        Ok(())
    }

    fn down() -> Result<(), spi::Error> {
        Table::drop().table(CharacterTrait::Table).run()?;
        Table::drop().table(TextRule::Table).run()?;
        Table::drop().table(NumericRule::Table).run()?;
        Table::drop().table(Trait::Table).run()?;
        Table::drop().table(Character::Table).run()?;
        Table::drop().table(FunField::Table).run()?;
        Table::drop().table(FunSchema::Table).run()?;
        Query::delete()
            .from_table(_Migration::Table)
            .and_where(Expr::col(_Migration::Name).eq(_230603095553Init.to_string()))
            .run()?;
        Ok(())
    }
}

#[pg_extern]
pub fn _230603095553_init_up() -> Result<(), spi::Error> {
    _230603095553Init::up()
}

#[pg_extern]
pub fn _230603095553_init_down() -> Result<(), spi::Error> {
    _230603095553Init::down()
}
