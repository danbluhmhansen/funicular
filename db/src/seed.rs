use crate::into_pgrx_arg::IntoPgrxArg;
use pgrx::prelude::*;

#[pg_extern]
pub fn fun_seed() -> Result<(), spi::Error> {
    let game = Spi::get_one::<pgrx::Uuid>(
        r#"INSERT INTO "game" ("name") VALUES ('foo') RETURNING "id";"#,
    )?;

    let skills = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "skill" ("game_id", "name") VALUES
                    ($1, 'str'),
                    ($1, 'dex'),
                    ($1, 'con'),
                    ($1, 'int'),
                    ($1, 'wis'),
                    ($1, 'cha'),
                    ($1, 'att_mel'),
                    ($1, 'att_fin'),
                    ($1, 'att_ran'),
                    ($1, 'att_thr')
                RETURNING "id";
                "#,
                None,
                Some(vec![game.unwrap().into_arg()]),
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    let str = skills[0].into_arg();
    let dex = skills[1].into_arg();
    let con = skills[2].into_arg();
    let int = skills[3].into_arg();
    let wis = skills[4].into_arg();
    let cha = skills[5].into_arg();
    let att_mel = skills[6].into_arg();
    let att_fin = skills[7].into_arg();
    let att_ran = skills[8].into_arg();
    let att_thr = skills[9].into_arg();

    Spi::run_with_args(
        r#"
        INSERT INTO "sub_skill" VALUES
            ($1, $3),
            ($1, $4),
            ($2, $5),
            ($2, $6);
        "#,
        Some(vec![str, dex, att_mel, att_fin, att_ran, att_thr]),
    )?;

    let traits = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "trait" ("name") VALUES
                    ('base'),
                    ('base_str'),
                    ('base_dex'),
                    ('base_con'),
                    ('base_int'),
                    ('base_wis'),
                    ('base_cha'),
                    ('dwarf'),
                    ('elf'),
                    ('att_mel'),
                    ('att_fin')
                RETURNING "id";
                "#,
                None,
                None,
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    let base = traits[0].into_arg();
    let base_str = traits[1].into_arg();
    let base_dex = traits[2].into_arg();
    let base_con = traits[3].into_arg();
    let base_int = traits[4].into_arg();
    let base_wis = traits[5].into_arg();
    let base_cha = traits[6].into_arg();
    let dwarf = traits[7].into_arg();
    let elf = traits[8].into_arg();
    let att_mel_trait = traits[9].into_arg();
    let att_fin_trait = traits[10].into_arg();

    Spi::run_with_args(
        r#"
        INSERT INTO "rule_num" VALUES
            ($1, $7,  8),
            ($2, $7,  8),
            ($3, $7,  8),
            ($4, $7,  8),
            ($5, $7,  8),
            ($6, $7,  8),
            ($1, $8,  1),
            ($2, $9,  1),
            ($3, $10, 1),
            ($4, $11, 1),
            ($5, $12, 1),
            ($6, $13, 1),
            ($1, $14, 2),
            ($2, $15, 2);
        "#,
        Some(vec![
            str, dex, con, int, wis, cha, base, base_str, base_dex, base_con, base_int, base_wis,
            base_cha, dwarf, elf,
        ]),
    )?;

    let actors = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "actor" ("name") VALUES
                    ('Braugnor Quickcleaver'),
                    ('Jaudenn Runecleaver')
                RETURNING "id";
                "#,
                None,
                None,
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    Spi::run_with_args(
        r#"
        INSERT INTO "actor_trait" VALUES
            ($1, $3,  1),
            ($1, $4,  6),
            ($1, $5,  6),
            ($1, $6,  7),
            ($1, $8,  3),
            ($1, $9,  2),
            ($1, $10, 1),
            ($2, $3,  1),
            ($2, $4,  3),
            ($2, $5,  6),
            ($2, $6,  6),
            ($2, $8,  7),
            ($2, $9,  2),
            ($2, $11, 1);
        "#,
        Some(vec![
            actors[0].into_arg(),
            actors[1].into_arg(),
            base,
            base_str,
            base_dex,
            base_con,
            base_int,
            base_wis,
            base_cha,
            dwarf,
            elf,
        ]),
    )?;

    let gears = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "gear" ("name") VALUES
                    ('warhammer'),
                    ('rapier')
                RETURNING "id";
                "#,
                None,
                None,
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    let warhammer = gears[0].into_arg();
    let rapier = gears[1].into_arg();

    Spi::run_with_args(
        r#"
        INSERT INTO "gear_trait" VALUES
            ($1, $3),
            ($2, $4);
        "#,
        Some(vec![warhammer, rapier, att_mel_trait, att_fin_trait]),
    )?;

    Spi::run_with_args(
        r#"
        INSERT INTO "actor_gear" VALUES
            ($1, $3),
            ($1, $4),
            ($2, $3),
            ($2, $4);
        "#,
        Some(vec![
            actors[0].into_arg(),
            actors[1].into_arg(),
            warhammer,
            rapier,
        ]),
    )?;

    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_fun_seed() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT fun_seed();")?;
        Ok(())
    }
}
