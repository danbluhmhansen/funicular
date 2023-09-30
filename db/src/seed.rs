use crate::into_pgrx_arg::IntoPgrxArg;
use pgrx::prelude::*;

fn seed_game() -> Result<pgrx::Uuid, spi::Error> {
    Ok(
        Spi::get_one::<pgrx::Uuid>(r#"INSERT INTO "game" ("name") VALUES ('Squirrels of Evil') RETURNING "id";"#)?
            .unwrap(),
    )
}

struct Skills {
    str: pgrx::Uuid,
    dex: pgrx::Uuid,
    con: pgrx::Uuid,
    int: pgrx::Uuid,
    wis: pgrx::Uuid,
    cha: pgrx::Uuid,
    att_mel: pgrx::Uuid,
    att_fin: pgrx::Uuid,
    att_ran: pgrx::Uuid,
    att_thr: pgrx::Uuid,
}

fn seed_skills(game: pgrx::Uuid) -> Result<Skills, spi::Error> {
    let skills = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "skill" ("game_id", "name") VALUES
                    ($1, 'Strength'),
                    ($1, 'Dexterity'),
                    ($1, 'Constitution'),
                    ($1, 'Intelligence'),
                    ($1, 'Wisdom'),
                    ($1, 'Charisma'),
                    ($1, 'Attack, Melee'),
                    ($1, 'Attack, Finesse'),
                    ($1, 'Attack, Ranged'),
                    ($1, 'Attack, Thrown')
                RETURNING "id";
                "#,
                None,
                Some(vec![game.into_arg()]),
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    let skills = Skills {
        str: skills[0],
        dex: skills[1],
        con: skills[2],
        int: skills[3],
        wis: skills[4],
        cha: skills[5],
        att_mel: skills[6],
        att_fin: skills[7],
        att_ran: skills[8],
        att_thr: skills[9],
    };

    Spi::run_with_args(
        r#"
        INSERT INTO "sub_skill" VALUES
            ($1, $3),
            ($1, $6),
            ($2, $4),
            ($2, $5);
        "#,
        Some(vec![
            skills.str.into_arg(),
            skills.dex.into_arg(),
            skills.att_mel.into_arg(),
            skills.att_fin.into_arg(),
            skills.att_ran.into_arg(),
            skills.att_thr.into_arg(),
        ]),
    )?;

    Ok(skills)
}

struct Traits {
    base: pgrx::Uuid,
    base_str: pgrx::Uuid,
    base_dex: pgrx::Uuid,
    base_con: pgrx::Uuid,
    base_int: pgrx::Uuid,
    base_wis: pgrx::Uuid,
    base_cha: pgrx::Uuid,
    dwarf: pgrx::Uuid,
    elf: pgrx::Uuid,
}

fn seed_traits(game: pgrx::Uuid) -> Result<Traits, spi::Error> {
    let traits = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "trait" ("game_id" ,"name") VALUES
                    ($1, 'Base'),
                    ($1, 'Base, Strength'),
                    ($1, 'Base, Dexterity'),
                    ($1, 'Base, Constitution'),
                    ($1, 'Base, Intelligence'),
                    ($1, 'Base, Wisdom'),
                    ($1, 'Base, Charisma'),
                    ($1, 'Dwarf'),
                    ($1, 'Elf'),
                    ($1, 'Attack, Melee'),
                    ($1, 'Attack, Finesse')
                RETURNING "id";
                "#,
                None,
                Some(vec![game.into_arg()]),
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    Ok(Traits {
        base: traits[0],
        base_str: traits[1],
        base_dex: traits[2],
        base_con: traits[3],
        base_int: traits[4],
        base_wis: traits[5],
        base_cha: traits[6],
        dwarf: traits[7],
        elf: traits[8],
    })
}

fn seed_rule_nums(skills: &Skills, traits: &Traits) -> Result<(), spi::Error> {
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
            skills.str.into_arg(),
            skills.dex.into_arg(),
            skills.con.into_arg(),
            skills.int.into_arg(),
            skills.wis.into_arg(),
            skills.cha.into_arg(),
            traits.base.into_arg(),
            traits.base_str.into_arg(),
            traits.base_dex.into_arg(),
            traits.base_con.into_arg(),
            traits.base_int.into_arg(),
            traits.base_wis.into_arg(),
            traits.base_cha.into_arg(),
            traits.dwarf.into_arg(),
            traits.elf.into_arg(),
        ]),
    )
}

fn seed_actors(game: pgrx::Uuid, skills: &Skills, traits: &Traits) -> Result<Vec<pgrx::Uuid>, spi::Error> {
    let kinds = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "actor_kind" ("game_id", "name") VALUES
                    ($1, 'Player')
                RETURNING "id";
                "#,
                None,
                Some(vec![game.into_arg()]),
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    Spi::run_with_args(
        r#"
        INSERT INTO "actor_skill" VALUES
            ($1, $2),
            ($1, $3),
            ($1, $4),
            ($1, $5),
            ($1, $6),
            ($1, $7);
        "#,
        Some(vec![
            kinds[0].into_arg(),
            skills.str.into_arg(),
            skills.dex.into_arg(),
            skills.con.into_arg(),
            skills.int.into_arg(),
            skills.wis.into_arg(),
            skills.cha.into_arg(),
        ]),
    )?;

    let actors = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "actor" ("kind_id", "name") VALUES
                    ($1, 'Braugnor Quickcleaver'),
                    ($1, 'Jaudenn Runecleaver')
                RETURNING "id";
                "#,
                None,
                Some(vec![kinds[0].into_arg()]),
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
            traits.base.into_arg(),
            traits.base_str.into_arg(),
            traits.base_dex.into_arg(),
            traits.base_con.into_arg(),
            traits.base_int.into_arg(),
            traits.base_wis.into_arg(),
            traits.base_cha.into_arg(),
            traits.dwarf.into_arg(),
            traits.elf.into_arg(),
        ]),
    )?;

    Ok(actors)
}

fn seed_gears(game: pgrx::Uuid, actor1: pgrx::Uuid, actor2: pgrx::Uuid, skills: &Skills) -> Result<(), spi::Error> {
    let kinds = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "gear_kind" ("game_id", "name") VALUES
                    ($1, 'Melee'),
                    ($1, 'Finesse'),
                    ($1, 'Ranged'),
                    ($1, 'Thrown')
                RETURNING "id";
                "#,
                None,
                Some(vec![game.into_arg()]),
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    let melee = kinds[0].into_arg();
    let finesse = kinds[1].into_arg();
    let ranged = kinds[2].into_arg();
    let thrown = kinds[3].into_arg();

    Spi::run_with_args(
        r#"
        INSERT INTO "gear_skill" VALUES
            ($1, $5),
            ($2, $5),
            ($2, $6),
            ($3, $7),
            ($4, $7),
            ($4, $8);
        "#,
        Some(vec![
            melee,
            finesse,
            ranged,
            thrown,
            skills.att_mel.into_arg(),
            skills.att_fin.into_arg(),
            skills.att_ran.into_arg(),
            skills.att_thr.into_arg(),
        ]),
    )?;

    let gears = Spi::connect(|mut client| -> Result<Vec<pgrx::Uuid>, spi::Error> {
        Ok(client
            .update(
                r#"
                INSERT INTO "gear" ("kind_id", "name") VALUES
                    ($1, 'Warhammer'),
                    ($2, 'Rapier')
                RETURNING "id";
                "#,
                None,
                Some(vec![melee, finesse]),
            )?
            .filter_map(|row| row.get(1).ok().flatten())
            .collect())
    })?;

    let warhammer = gears[0].into_arg();
    let rapier = gears[1].into_arg();

    Spi::run_with_args(
        r#"
        INSERT INTO "actor_gear" VALUES
            ($1, $3),
            ($1, $4),
            ($2, $3),
            ($2, $4);
        "#,
        Some(vec![actor1.into_arg(), actor2.into_arg(), warhammer, rapier]),
    )?;

    Ok(())
}

#[pg_extern]
pub fn fun_seed() -> Result<(), spi::Error> {
    let game = seed_game()?;
    let skills = seed_skills(game)?;
    let traits = seed_traits(game)?;
    seed_rule_nums(&skills, &traits)?;
    let actors = seed_actors(game, &skills, &traits)?;
    seed_gears(game, actors[0], actors[1], &skills)?;
    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_fun_seed() -> Result<(), spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT fun_seed();")
    }
}
