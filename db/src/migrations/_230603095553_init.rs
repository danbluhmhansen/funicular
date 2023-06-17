use crate::{
    migrations::Migration, models::Character, models::CharacterNumericField,
    models::CharacterTrait, models::Field, models::NumericRule, models::Schema, models::TextRule,
    models::Trait, models::_Migration,
};
use pgrx::prelude::*;

struct _230603095553Init;

impl Migration for _230603095553Init {
    fn up() -> Result<(), spi::Error> {
        Spi::run(r#"COMMENT ON SCHEMA public IS e'@graphql({"inflect_names": true})';"#)?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {schema} (
                {schema_id} uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                {schema_name} text NOT NULL CHECK ({schema_name} ~ '^[a-z_]*$')
            );
            "#,
            schema = Schema::Table,
            schema_id = Schema::Id,
            schema_name = Schema::Name,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'Collection of fields.';",
            Schema::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {field} (
                {field_id} uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                {field_schema_id} uuid NOT NULL REFERENCES {schema}({schema_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {field_field_id} uuid REFERENCES {field}({field_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {field_name} text NOT NULL CHECK ({field_name} ~ '^[a-z_]*$'),
                {field_description} text
            );
            "#,
            field = Field::Table,
            field_id = Field::Id,
            field_schema_id = Field::SchemaId,
            field_field_id = Field::FieldId,
            field_name = Field::Name,
            field_description = Field::Description,
            schema = Schema::Table,
            schema_id = Schema::Id,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'Describes a specific attribute of a character or item.';",
            Field::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {character} (
                {character_id} uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                {character_name} text NOT NULL
            );
            "#,
            character = Character::Table,
            character_id = Character::Id,
            character_name = Character::Name,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'An individual controlled by a player or Game Master.';",
            Character::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {trait} (
                {trait_id} uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
                {trait_name} text NOT NULL
            );
            "#,
            trait = Trait::Table,
            trait_id = Trait::Id,
            trait_name = Trait::Name,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'Describes a specific trait or effect of a character or item.';",
            Trait::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {numeric_rule} (
                {numeric_rule_field_id} uuid NOT NULL REFERENCES {field}({field_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {numeric_rule_trait_id} uuid NOT NULL REFERENCES {trait}({trait_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {numeric_rule_value} numeric NOT NULL,
                PRIMARY KEY ({numeric_rule_field_id}, {numeric_rule_trait_id})
            );
            "#,
            numeric_rule = NumericRule::Table,
            numeric_rule_field_id = NumericRule::FieldId,
            numeric_rule_trait_id = NumericRule::TraitId,
            numeric_rule_value = NumericRule::Value,
            field = Field::Table,
            field_id = Field::Id,
            trait = Trait::Table,
            trait_id = Trait::Id,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'Describes a rule of a trait, which field to affect and by what amount';",
            NumericRule::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {text_rule} (
                {text_rule_field_id} uuid NOT NULL REFERENCES {field}({field_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {text_rule_trait_id} uuid NOT NULL REFERENCES {trait}({trait_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {text_rule_value} numeric NOT NULL,
                PRIMARY KEY ({text_rule_field_id}, {text_rule_trait_id})
            );
            "#,
            text_rule = TextRule::Table,
            text_rule_field_id = TextRule::FieldId,
            text_rule_trait_id = TextRule::TraitId,
            text_rule_value = TextRule::Value,
            field = Field::Table,
            field_id = Field::Id,
            trait = Trait::Table,
            trait_id = Trait::Id,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'Describes a rule of a trait, which field to affect and by what amount';",
            TextRule::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE TABLE {char_trait} (
                {char_trait_character_id} uuid NOT NULL REFERENCES {character}({character_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                {char_trait_trait_id} uuid NOT NULL REFERENCES {trait}({trait_id}) ON DELETE CASCADE ON UPDATE CASCADE,
                PRIMARY KEY ({char_trait_character_id}, {char_trait_trait_id})
            );
            "#,
            char_trait = CharacterTrait::Table,
            char_trait_character_id = CharacterTrait::CharacterId,
            char_trait_trait_id = CharacterTrait::TraitId,
            character = Character::Table,
            character_id = Character::Id,
            trait = Trait::Table,
            trait_id = Trait::Id,
        ))?;

        Spi::run(&format!(
            "COMMENT ON TABLE {} IS 'Connection between characters and traits.';",
            CharacterTrait::Table
        ))?;

        Spi::run(&format!(
            r#"
            CREATE VIEW {char_num_field} AS
            SELECT
            	{char}.{char_id} AS {char_num_field_char_id},
            	{field}.{field_id} AS {char_num_field_field_id},
            	SUM({numeric_rule}.{numeric_rule_value}) AS {char_num_field_value}
            FROM {field}
            JOIN {numeric_rule} ON {numeric_rule}.{numeric_rule_field_id} = {field}.{field_id}
            JOIN {trait} ON {trait}.{trait_id} = {numeric_rule}.{numeric_rule_trait_id}
            JOIN {char_trait} ON {char_trait}.{char_trait_trait_id} = {trait}.{trait_id}
            JOIN {char} ON {char}.{char_id} = {char_trait}.{char_trait_char_id}
            GROUP BY {field}.{field_id}, {char}.{char_id}
            ORDER BY {char}.{char_id};
            "#,
            char_num_field = CharacterNumericField::View,
            char_num_field_char_id = CharacterNumericField::CharacterId,
            char_num_field_field_id = CharacterNumericField::FieldId,
            char_num_field_value = CharacterNumericField::Value,
            field = Field::Table,
            field_id = Field::Id,
            char = Character::Table,
            char_id = Character::Id,
            trait = Trait::Table,
            trait_id = Trait::Id,
            numeric_rule = NumericRule::Table,
            numeric_rule_field_id = NumericRule::FieldId,
            numeric_rule_trait_id = NumericRule::TraitId,
            numeric_rule_value = NumericRule::Value,
            char_trait = CharacterTrait::Table,
            char_trait_char_id = CharacterTrait::CharacterId,
            char_trait_trait_id = CharacterTrait::TraitId,
        ))?;

        Spi::run(&format!(
            r#"
            COMMENT ON VIEW {char_num_field} IS E'
                @graphql({{
                    "primary_key_columns": [{char_num_field_char_id},{char_num_field_field_id}],
                    "foreign_keys": [
                        {{
                            "local_columns": [{char_num_field_char_id}],
                            "foreign_schema": "public",
                            "foreign_table": {char},
                            "foreign_columns": [{char_id}]
                        }},
                        {{
                            "local_columns": [{char_num_field_field_id}],
                            "foreign_schema": "public",
                            "foreign_table": {field},
                            "foreign_columns": [{field_id}]
                        }}
                    ]
                }})';
            "#,
            char_num_field = CharacterNumericField::View,
            char_num_field_char_id = CharacterNumericField::CharacterId,
            char_num_field_field_id = CharacterNumericField::FieldId,
            char = Character::Table,
            char_id = Character::Id,
            field = Field::Table,
            field_id = Field::Id,
        ))?;

        Spi::run(&format!(
            "INSERT INTO {} VALUES ('230603095553_init');",
            _Migration::Table
        ))?;

        Ok(())
    }

    fn down() -> Result<(), spi::Error> {
        Spi::run(&format!("DROP VIEW {};", CharacterNumericField::View))?;
        Spi::run(&format!("DROP TABLE {};", CharacterTrait::Table))?;
        Spi::run(&format!("DROP TABLE {};", TextRule::Table))?;
        Spi::run(&format!("DROP TABLE {};", NumericRule::Table))?;
        Spi::run(&format!("DROP TABLE {};", Trait::Table))?;
        Spi::run(&format!("DROP TABLE {};", Character::Table))?;
        Spi::run(&format!("DROP TABLE {};", Field::Table))?;
        Spi::run(&format!("DROP TABLE {};", Schema::Table))?;
        Spi::run(&format!(
            "DELETE FROM {migration} WHERE {migration_name} = '230603095553_init';",
            migration = _Migration::Table,
            migration_name = _Migration::Name
        ))?;
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
