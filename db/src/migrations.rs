use funicular_derive::pg_migration;
use pgrx::prelude::*;

#[pg_extern]
pub fn _000000000000_migrations_up() -> Result<(), spi::Error> {
    if !Spi::get_one_with_args::<bool>(
            r#"SELECT 1::boolean FROM "information_schema"."tables" WHERE "table_schema" = $1 AND "table_name" = $2;"#,
            vec![
                (PgBuiltInOids::TEXTOID.oid(), "public".into_datum()),
                (PgBuiltInOids::TEXTOID.oid(), "_migration".into_datum()),
            ],
        ).is_ok_and(|o| !o.is_some_and(|b| !b)) {
            Spi::run(include_str!("migrations/_000000000000_migrations/up.sql"))?;
        }
    Ok(())
}

#[pg_extern]
pub fn _000000000000_migrations_down() -> Result<(), spi::Error> {
    if Spi::get_one_with_args::<bool>(
        r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
        vec![(
            PgBuiltInOids::TEXTOID.oid(),
            "000000000000_migrations".into_datum(),
        )],
    )
    .is_ok_and(|o| o.is_some_and(|b| b))
    {
        Spi::run(include_str!("migrations/_000000000000_migrations/down.sql"))?;
    }
    Ok(())
}

pg_migration!(_230625095802_auth);
pg_migration!(_230625095922_game);
pg_migration!(_230625105931_actor);

#[pg_extern]
fn migrations_up() -> Result<(), spi::Error> {
    Spi::run("SELECT _000000000000_migrations_up();")?;
    Spi::run("SELECT _230625095802_auth_up();")?;
    Spi::run("SELECT _230625095922_game_up();")?;
    Spi::run("SELECT _230625105931_actor_up();")?;
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

    Spi::run("SELECT _230625105931_actor_down();")?;
    Spi::run("SELECT _230625095922_game_down();")?;
    Spi::run("SELECT _230625095802_auth_down();")?;
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