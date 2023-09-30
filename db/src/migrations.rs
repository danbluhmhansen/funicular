use pgrx::prelude::*;

#[pg_extern]
fn _20230910180918_init() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230910180918_init.sql"))
}

#[pg_extern]
fn _20230910184712_actor_num_skill() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230910184712_actor_num_skill.sql"))
}

#[pg_extern]
fn _20230910184717_gear_num_skill() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230910184717_gear_num_skill.sql"))
}

#[pg_extern]
fn _20230930093227_actor_skill_agg() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230930093227_actor_skill_agg.sql"))
}

#[pg_extern]
fn migrations_up() -> Result<(), spi::Error> {
    Spi::run("SELECT _20230910180918_init();")?;
    Spi::run("SELECT _20230910184712_actor_num_skill();")?;
    Spi::run("SELECT _20230910184717_gear_num_skill();")?;
    Spi::run("SELECT _20230930093227_actor_skill_agg();")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_migrations_up() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")
    }
}
