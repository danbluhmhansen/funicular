use pgrx::prelude::*;

use pgrx::Uuid;

fn select_schema_name(schema_id: Uuid) -> Result<Option<String>, pgrx::spi::Error> {
    Spi::get_one_with_args::<String>(
        "SELECT name FROM schema WHERE id = $1;",
        vec![(PgBuiltInOids::UUIDOID.oid(), schema_id.into_datum())],
    )
}

fn select_schema_field_cols(schema_id: Uuid) -> String {
    match Spi::connect(|client| -> Result<Vec<String>, pgrx::spi::Error> {
        client
            .select(
                "SELECT path, fun_type FROM schema_field WHERE schema_id = $1 ORDER BY path;",
                None,
                Some(vec![(PgBuiltInOids::UUIDOID.oid(), schema_id.into_datum())]),
            )?
            .map(|row| -> Result<String, pgrx::spi::Error> {
                match row["path"].value::<String>() {
                    Ok(Some(path)) => match row["fun_type"].value() {
                        Ok(Some("int")) => Ok(format!("{path} bigint",)),
                        _ => Ok(format!("{path} bigint",)),
                    },
                    _ => Ok("".to_string()),
                }
            })
            .collect::<Result<Vec<String>, pgrx::spi::Error>>()
    }) {
        Ok(mut cols) => {
            cols.insert(0, "name text".to_string());
            cols.join(", ")
        }
        _ => "".to_string(),
    }
}

#[pg_extern]
pub fn refresh_char_aggr(schema_id: Uuid) -> Result<(), pgrx::spi::Error> {
    if let Ok(Some(view_name)) = select_schema_name(schema_id) {
        let cols = select_schema_field_cols(schema_id);
        Spi::run(&format!(
            "DROP MATERIALIZED VIEW IF EXISTS char_aggr_{view_name};",
        ))?;
        Spi::run(&format!(
            r#"
                CREATE MATERIALIZED VIEW char_aggr_{view_name} AS
                    SELECT * FROM crosstab('
                        SELECT
                            char.name,
                            schema_field.path,
                            SUM(CASE
                                WHEN schema_field.fun_type = ''int'' THEN effect.val::int
                                ELSE 0
                            END)
                        FROM char
                        JOIN schema_field ON true
                        JOIN char_trait ON char_trait.char_id = char.id
                        JOIN trait ON trait.id = char_trait.trait_id
                        LEFT JOIN effect ON effect.trait_id = trait.id AND effect.schema_field_id = schema_field.id
                        WHERE schema_field.schema_id = ''{schema_id}''
                        GROUP BY char.id, schema_field.id
                        ORDER BY 1, 2'
                    ) AS ct({cols});
            "#,
        ))?;
    }
    Ok(())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::{prelude::*, Uuid, UuidBytes};

    const SCHEMA_ID: UuidBytes = [
        0x31, 0x2c, 0x5a, 0xc5, 0x23, 0xaa, 0x45, 0x68, 0x9d, 0x10, 0x59, 0x49, 0x65, 0x0b, 0xc8,
        0xc0,
    ];

    #[pg_test]
    fn test_select_schema_name() -> Result<(), pgrx::spi::Error> {
        assert_eq!(
            "foo".to_string(),
            crate::refresh_char_aggr::select_schema_name(Uuid::from_bytes(SCHEMA_ID))?.unwrap()
        );
        Ok(())
    }

    #[pg_test]
    fn test_select_schema_field_cols() {
        assert_eq!(
            "name text, charisma bigint, constitution bigint, dexterity bigint, intelligence bigint, strength bigint, wisdom bigint".to_string(),
            crate::refresh_char_aggr::select_schema_field_cols(Uuid::from_bytes(SCHEMA_ID))
        )
    }

    #[pg_test]
    fn test_refresh_char_aggr() -> Result<(), pgrx::spi::Error> {
        Spi::run("CREATE EXTENSION tablefunc;")?;
        crate::refresh_char_aggr::refresh_char_aggr(Uuid::from_bytes(SCHEMA_ID))?;
        assert_eq!(
            "Braugnor Quickcleaver".to_string(),
            Spi::get_one::<String>("SELECT name FROM char_aggr_foo;")?.unwrap()
        );
        Ok(())
    }
}
