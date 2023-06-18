use crate::migrations::Migration;
use pgrx::prelude::*;

struct _000000000000Migrations;

impl Migration for _000000000000Migrations {
    fn up() -> Result<(), pgrx::spi::Error> {
        Spi::run(include_str!("up.sql"))
    }

    fn down() -> Result<(), pgrx::spi::Error> {
        Spi::run(include_str!("down.sql"))
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
