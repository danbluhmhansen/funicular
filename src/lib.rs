use pgrx::{prelude::*, Uuid};

pgrx::pg_module_magic!();

extension_sql_file!("../static/up.sql");

#[pg_extern]
fn hello_funicular_ext() -> &'static str {
    "Hello, funicular_ext"
}

#[pg_extern]
fn select_schema_name(schema_id: Uuid) -> Result<Option<String>, pgrx::spi::Error> {
    Spi::get_one_with_args::<String>(
        "SELECT name FROM schema WHERE id = $1;",
        vec![(PgBuiltInOids::UUIDOID.oid(), schema_id.into_datum())],
    )
}

#[pg_extern]
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
                        Ok(Some("int")) => Ok(format!("{} bigint", path)),
                        _ => Ok(format!("{} bigint", path)),
                    },
                    _ => Ok("".to_string()),
                }
            })
            .collect::<Result<Vec<String>, pgrx::spi::Error>>()
    }) {
        Ok(cols) => cols.join(", "),
        _ => "".to_string(),
    }
}

#[pg_extern]
fn refresh_char_aggr(schema_id: Uuid) -> Result<(), pgrx::spi::Error> {
    if let Ok(Some(view_name)) = select_schema_name(schema_id) {
        let cols = select_schema_field_cols(schema_id);
        Spi::run(&format!("DROP MATERIALIZED VIEW IF EXISTS {}", view_name))?;
        Spi::run(&format!(
            r#"
                CREATE MATERIALIZED VIEW {} AS
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
                        GROUP BY char.id, schema_field.id
                        ORDER BY 1, 2'
                    ) AS ct(name text, {});
            "#,
            view_name, cols
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
    fn test_hello_funicular_ext() {
        assert_eq!("Hello, funicular_ext", crate::hello_funicular_ext());
    }

    #[pg_test]
    fn test_select_schema_name() {
        assert_eq!(
            "foo".to_string(),
            crate::select_schema_name(Uuid::from_bytes(SCHEMA_ID))
                .unwrap()
                .unwrap()
        )
    }

    #[pg_test]
    fn test_refresh_char_aggr() {
        Spi::run("CREATE EXTENSION tablefunc;").unwrap();
        crate::refresh_char_aggr(Uuid::from_bytes(SCHEMA_ID)).unwrap();
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
