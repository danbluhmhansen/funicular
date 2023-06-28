use funicular_derive::pg_migration;
use pgrx::prelude::*;

fn migration_table_exists() -> bool {
    Spi::get_one_with_args::<bool>(
        r#"SELECT EXISTS(SELECT 1 FROM "information_schema"."tables" WHERE "table_schema" = $1 AND "table_name" = $2);"#,
        vec![
            (PgBuiltInOids::TEXTOID.oid(), "public".into_datum()),
            (PgBuiltInOids::TEXTOID.oid(), "_migration".into_datum()),
        ],
    ).is_ok_and(|o| o.is_some_and(|b| b))
}

#[pg_extern]
fn _000000000000_migrations_up() -> Result<(), spi::Error> {
    if !migration_table_exists() {
        Spi::run(include_str!("migrations/_000000000000_migrations/up.sql"))?;
    }
    Ok(())
}

#[pg_extern]
fn _000000000000_migrations_down() -> Result<(), spi::Error> {
    if migration_table_exists() {
        Spi::run(include_str!("migrations/_000000000000_migrations/down.sql"))?;
    }
    Ok(())
}

pg_migration!(_230625095802_auth);
pg_migration!(_230625095922_game);
pg_migration!(_230625105931_actor);
pg_migration!(_230628175736_gear);

#[pg_extern]
fn migrations_up() -> Result<(), spi::Error> {
    Spi::run("SELECT _000000000000_migrations_up();")?;
    Spi::run("SELECT _230625095802_auth_up();")?;
    Spi::run("SELECT _230625095922_game_up();")?;
    Spi::run("SELECT _230625105931_actor_up();")?;
    Spi::run("SELECT _230628175736_gear_up();")?;
    Ok(())
}

#[pg_extern]
fn migrations_down() -> Result<(), spi::Error> {
    if migration_table_exists() {
        Spi::run("SELECT _230628175736_gear_down();")?;
        Spi::run("SELECT _230625105931_actor_down();")?;
        Spi::run("SELECT _230625095922_game_down();")?;
        Spi::run("SELECT _230625095802_auth_down();")?;
        Spi::run("SELECT _000000000000_migrations_down();")?;
    }

    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_migrations_up() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;

        let table_count = Spi::get_one::<i64>(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';",
        )?;
        assert_ne!(table_count, Some(0));

        Spi::run("SELECT migrations_up();")?;
        assert_eq!(
            Spi::get_one::<i64>(
                "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';",
            )?,
            table_count
        );

        Ok(())
    }

    #[pg_test]
    fn test_migrations_down() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();").unwrap();
        Spi::run("SELECT migrations_down();").unwrap();

        let table_count = Spi::get_one::<i64>(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';",
        )
        .unwrap();
        assert_eq!(table_count, Some(0));

        Spi::run("SELECT migrations_down();").unwrap();
        assert_eq!(table_count, Some(0));

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
