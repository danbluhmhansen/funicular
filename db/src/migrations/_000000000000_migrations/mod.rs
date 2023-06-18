use crate::migrations::Migration;
use pgrx::prelude::*;

struct _000000000000Migrations;

impl Migration for _000000000000Migrations {
    fn up() -> Result<(), pgrx::spi::Error> {
        if !Spi::get_one_with_args::<bool>(
            r#"SELECT 1::boolean FROM "information_schema"."tables" WHERE "table_schema" = $1 AND "table_name" = $2;"#,
            vec![
                (PgBuiltInOids::TEXTOID.oid(), "public".into_datum()),
                (PgBuiltInOids::TEXTOID.oid(), "_migration".into_datum()),
            ],
        ).is_ok_and(|o| !o.is_some_and(|b| !b)) {
            Spi::run(include_str!("up.sql"))?;
        }
        Ok(())
    }

    fn down() -> Result<(), pgrx::spi::Error> {
        if Spi::get_one_with_args::<bool>(
            r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                "000000000000_migrations".into_datum(),
            )],
        )
        .is_ok_and(|o| o.is_some_and(|b| b))
        {
            Spi::run(include_str!("down.sql"))?;
        }
        Ok(())
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
