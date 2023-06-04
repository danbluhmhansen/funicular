use crate::{
    migrations::Migration,
    models::{Char, CharTrait, Effect, GenRandUuid7, Schema, SchemaField, Trait, _Migration},
    sea_ext::SeaRunExt,
};
use pgrx::prelude::*;
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Func, Index, Query, Table};

struct _230603095553Init;

impl Migration for _230603095553Init {
    fn up() -> Result<(), spi::Error> {
        Table::create()
            .table(_Migration::Table)
            .col(ColumnDef::new(_Migration::Name).text().primary_key())
            .run()?;

        Table::create()
            .table(Schema::Table)
            .col(
                ColumnDef::new(Schema::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(
                ColumnDef::new(Schema::Name)
                    .text()
                    .not_null()
                    .unique_key()
                    .extra("CHECK (name ~ '^[a-z_]*$')".to_string()),
            )
            .run()?;

        Table::create()
            .table(SchemaField::Table)
            .col(
                ColumnDef::new(SchemaField::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(ColumnDef::new(SchemaField::SchemaId).uuid().not_null())
            .col(
                ColumnDef::new(SchemaField::Path)
                    .text()
                    .not_null()
                    .extra("CHECK (path ~ '^[a-z_]*$')".to_string()),
            )
            .col(
                ColumnDef::new(SchemaField::FunType)
                    .extra("funtype".to_string())
                    .not_null(),
            )
            .col(ColumnDef::new(SchemaField::Desc).text())
            .foreign_key(
                ForeignKey::create()
                    .from(SchemaField::Table, SchemaField::SchemaId)
                    .to(Schema::Table, Schema::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Table::create()
            .table(Char::Table)
            .col(
                ColumnDef::new(Char::Id)
                    .uuid()
                    .primary_key()
                    .default(Func::cust(GenRandUuid7)),
            )
            .col(ColumnDef::new(Char::Name).text().not_null())
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
            .table(Effect::Table)
            .col(ColumnDef::new(Effect::TraitId).uuid().not_null())
            .col(ColumnDef::new(Effect::SchemaFieldId).uuid().not_null())
            .col(ColumnDef::new(Effect::Val).text().not_null())
            .primary_key(
                Index::create()
                    .col(Effect::TraitId)
                    .col(Effect::SchemaFieldId),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Effect::Table, Effect::TraitId)
                    .to(Trait::Table, Trait::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Effect::Table, Effect::SchemaFieldId)
                    .to(SchemaField::Table, SchemaField::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Table::create()
            .table(CharTrait::Table)
            .col(ColumnDef::new(CharTrait::CharId).uuid().not_null())
            .col(ColumnDef::new(CharTrait::TraitId).uuid().not_null())
            .primary_key(
                Index::create()
                    .col(CharTrait::CharId)
                    .col(CharTrait::TraitId),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CharTrait::Table, CharTrait::CharId)
                    .to(Char::Table, Char::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CharTrait::Table, CharTrait::TraitId)
                    .to(Trait::Table, Trait::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .run()?;

        Query::insert()
            .into_table(_Migration::Table)
            .columns([_Migration::Name])
            .values_panic(["_230603095553Init".into()])
            .run()?;

        Ok(())
    }

    fn down() -> Result<(), spi::Error> {
        Table::drop().table(CharTrait::Table).run()?;
        Table::drop().table(Effect::Table).run()?;
        Table::drop().table(Trait::Table).run()?;
        Table::drop().table(Char::Table).run()?;
        Table::drop().table(SchemaField::Table).run()?;
        Table::drop().table(Schema::Table).run()?;
        Table::drop().table(_Migration::Table).run()?;
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
