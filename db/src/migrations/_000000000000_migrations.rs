use crate::{migrations::Migration, models::_Migration};
use pgrx::prelude::*;

struct _000000000000Migrations;

impl Migration for _000000000000Migrations {
    fn up() -> Result<(), pgrx::spi::Error> {
        Spi::run(&format!(
            "CREATE TABLE {migration} ({migration_name} text PRIMARY KEY);",
            migration = _Migration::Table,
            migration_name = _Migration::Name
        ))?;
        Spi::run(&format!(
            "INSERT INTO {} VALUES ('000000000000_migrations');",
            _Migration::Table
        ))?;
        Ok(())
    }

    fn down() -> Result<(), pgrx::spi::Error> {
        Spi::run(&format!("DROP TABLE {};", _Migration::Table))
    }
}

#[pg_extern]
pub fn _000000000000_migrations_up() -> Result<(), spi::Error> {
    _000000000000Migrations::up()
}

#[pg_extern]
pub fn _000000000000_migrations_down() -> Result<(), spi::Error> {
    _000000000000Migrations::down()
}
