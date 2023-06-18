use pgrx::prelude::*;

mod _000000000000_migrations;
mod _230603095553_init;
mod _230618102627_auth;

trait Migration {
    fn up() -> Result<(), spi::Error>;
    fn down() -> Result<(), spi::Error>;
}

#[pg_extern]
fn migrations_up() -> Result<(), spi::Error> {
    Spi::run("SELECT _000000000000_migrations_up();")?;
    Spi::run("SELECT _230603095553_init_up();")?;
    Spi::run("SELECT _230618102627_auth_up();")?;
    Ok(())
}

#[pg_extern]
fn migrations_down() -> Result<(), spi::Error> {
    if !Spi::get_one_with_args::<bool>(
        r#"SELECT 1::boolean FROM "information_schema"."tables" WHERE "table_schema" = $1 AND "table_name" = $2;"#,
        vec![(
            PgBuiltInOids::TEXTOID.oid(),
            "public".into_datum(),
        ),
            (
            PgBuiltInOids::TEXTOID.oid(),
            "_migrations".into_datum(),
        )],
    )
    .is_ok_and(|o| !o.is_some_and(|b| !b))
    {
        return Ok(());
    }

    Spi::run("SELECT _230618102627_auth_down();")?;
    Spi::run("SELECT _230603095553_init_down();")?;
    Spi::run("SELECT _000000000000_migrations_down();")?;

    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_migrations_up() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT migrations_up();")?;
        Ok(())
    }

    #[pg_test]
    fn test_migrations_down() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_down();")?;
        Spi::run("SELECT migrations_down();")?;
        Ok(())
    }

    #[pg_test]
    fn test_migrations_up_down() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT migrations_down();")?;
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT migrations_down();")?;
        Ok(())
    }
}
