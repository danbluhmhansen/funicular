use pgrx::prelude::*;

#[pg_extern]
fn _20230910180918_init() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230910180918_init.sql"))?;
    Ok(())
}

#[pg_extern]
fn _20230910184712_actor_num_skill() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230910184712_actor_num_skill.sql"))?;
    Ok(())
}

#[pg_extern]
fn _20230910184717_gear_num_skill() -> Result<(), spi::Error> {
    Spi::run(include_str!("migrations/20230910184717_gear_num_skill.sql"))?;
    Ok(())
}

#[pg_extern]
fn migrations_up() -> Result<(), spi::Error> {
    Spi::run("SELECT _20230910180918_init();")?;
    Spi::run("SELECT _20230910184712_actor_num_skill();")?;
    Spi::run("SELECT _20230910184717_gear_num_skill();")?;
    Ok(())
}
