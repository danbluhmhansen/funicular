use crate::migrations::Migration;
use pgrx::prelude::*;

struct _230603095553Init;

impl Migration for _230603095553Init {
    fn up() -> Result<(), spi::Error> {
        Spi::run(
            r#"
            CREATE TABLE schema (
                id uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                name text NOT NULL CHECK (name ~ '^[a-z_]*$')
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE TABLE field (
                id uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                schema_id uuid NOT NULL REFERENCES schema(id) ON DELETE CASCADE ON UPDATE CASCADE,
                field_id uuid REFERENCES field(id) ON DELETE CASCADE ON UPDATE CASCADE,
                name text NOT NULL CHECK (name ~ '^[a-z_]*$'),
                description text
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE TABLE character (
                id uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                name text NOT NULL
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE TABLE trait (
                id uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                name text NOT NULL
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE TABLE numeric_rule (
                field_id uuid NOT NULL REFERENCES field(id) ON DELETE CASCADE ON UPDATE CASCADE,
                trait_id uuid NOT NULL REFERENCES trait(id) ON DELETE CASCADE ON UPDATE CASCADE,
                value numeric NOT NULL,
                PRIMARY KEY (field_id, trait_id)
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE TABLE text_rule (
                field_id uuid NOT NULL REFERENCES field(id) ON DELETE CASCADE ON UPDATE CASCADE,
                trait_id uuid NOT NULL REFERENCES trait(id) ON DELETE CASCADE ON UPDATE CASCADE,
                value text NOT NULL,
                PRIMARY KEY (field_id, trait_id)
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE TABLE character_trait (
                character_id uuid NOT NULL REFERENCES character(id) ON DELETE CASCADE ON UPDATE CASCADE,
                trait_id uuid NOT NULL REFERENCES trait(id) ON DELETE CASCADE ON UPDATE CASCADE
            );
            "#,
        )?;

        Spi::run(
            r#"
            CREATE VIEW character_numeric_field AS
            SELECT
            	character.id AS character_id,
            	field.id AS field_id,
            	SUM(numeric_rule.value)
            FROM field
            JOIN numeric_rule ON numeric_rule.field_id = field.id
            JOIN trait ON trait.id = numeric_rule.trait_id
            JOIN character_trait ON character_trait.trait_id = trait.id
            JOIN character ON character.id = character_trait.character_id
            GROUP BY field.id, character.id
            ORDER BY character.id;
            "#,
        )?;

        Spi::run("INSERT INTO _migration VALUES ('230603095553_init');")?;

        Ok(())
    }

    fn down() -> Result<(), spi::Error> {
        Spi::run("DROP VIEW character_numeric_field;")?;
        Spi::run("DROP TABLE character_trait;")?;
        Spi::run("DROP TABLE text_rule;")?;
        Spi::run("DROP TABLE numeric_rule;")?;
        Spi::run("DROP TABLE trait;")?;
        Spi::run("DROP TABLE character;")?;
        Spi::run("DROP TABLE field;")?;
        Spi::run("DROP TABLE schema;")?;
        Spi::run("DELETE FROM _migration WHERE name = '230603095553_init';")?;
        Ok(())
    }
}

#[pg_extern]
pub fn _230603095553_init_up() -> Result<(), spi::Error> {
    _230603095553Init::up()
}

#[pg_extern]
pub fn _230603095553_init_down() -> Result<(), spi::Error> {
    _230603095553Init::down()
}
