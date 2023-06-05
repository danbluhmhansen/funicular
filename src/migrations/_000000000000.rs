use crate::{migrations::Migration, models::_Migration, sea_ext::SeaRunExt};
use pgrx::prelude::*;
use sea_query::{ColumnDef, Query, Table};

struct _000000000000;

impl Migration for _000000000000 {
    fn up() -> Result<(), pgrx::spi::Error> {
        Table::create()
            .table(_Migration::Table)
            .col(ColumnDef::new(_Migration::Name).text().primary_key())
            .run()?;
        Query::insert()
            .into_table(_Migration::Table)
            .columns([_Migration::Name])
            .values_panic(["_000000000000".into()])
            .run()?;
        Ok(())
    }

    fn down() -> Result<(), pgrx::spi::Error> {
        Table::drop().table(_Migration::Table).run()?;
        Ok(())
    }
}

#[pg_extern]
pub fn _000000000000_up() -> Result<(), spi::Error> {
    _000000000000::up()
}

#[pg_extern]
pub fn _000000000000_down() -> Result<(), spi::Error> {
    _000000000000::down()
}
