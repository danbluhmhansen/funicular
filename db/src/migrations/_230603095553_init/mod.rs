use crate::migrations::Migration;
use pgrx::prelude::*;

struct _230603095553Init;

impl Migration for _230603095553Init {
    fn up() -> Result<(), spi::Error> {
        Spi::run(include_str!("up.sql"))
    }

    fn down() -> Result<(), spi::Error> {
        Spi::run(include_str!("down.sql"))
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
