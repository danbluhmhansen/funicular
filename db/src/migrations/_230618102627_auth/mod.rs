use crate::migrations::Migration;
use pgrx::prelude::*;

struct _230618102627Auth;

impl Migration for _230618102627Auth {
    fn up() -> Result<(), spi::Error> {
        if !Spi::get_one_with_args::<bool>(
            r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                "230618102627_auth".into_datum(),
            )],
        )
        .is_ok_and(|o| !o.is_some_and(|b| !b))
        {
            Spi::run(include_str!("up.sql"))?;
        }
        Ok(())
    }

    fn down() -> Result<(), spi::Error> {
        if Spi::get_one_with_args::<bool>(
            r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                "230618102627_auth".into_datum(),
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
pub fn _230618102627_auth_up() -> Result<(), spi::Error> {
    _230618102627Auth::up()
}

#[pg_extern]
pub fn _230618102627_auth_down() -> Result<(), spi::Error> {
    _230618102627Auth::down()
}
