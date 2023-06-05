use pgrx::prelude::*;

mod _000000000000;
mod _230603095553_init;

trait Migration {
    fn up() -> Result<(), spi::Error>;
    fn down() -> Result<(), spi::Error>;
}

#[pg_extern]
fn migrations_up() -> Result<(), spi::Error> {
    Spi::run("SELECT _000000000000_up();")?;
    Spi::run("SELECT _230603095553_init_up();")?;
    Ok(())
}

#[pg_extern]
fn migrations_down() -> Result<(), spi::Error> {
    Spi::run("SELECT _230603095553_init_down();")?;
    Spi::run("SELECT _000000000000_down();")?;
    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_migrations() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT migrations_down();")?;
        Ok(())
    }
}
