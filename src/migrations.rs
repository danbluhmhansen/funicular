use pgrx::prelude::*;

mod _230603095553_init;

trait Migration {
    fn up() -> Result<(), spi::Error>;
    fn down() -> Result<(), spi::Error>;
}
