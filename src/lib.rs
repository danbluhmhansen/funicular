use std::{error::Error, iter::zip};

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
        Spi::run(&format!(
            "DROP MATERIALIZED VIEW IF EXISTS char_aggr_{}",
            view_name
        ))?;
        Spi::run(&format!(
            r#"
                CREATE MATERIALIZED VIEW char_aggr_{} AS
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

#[pg_trigger]
fn refresh_char_aggr_trigger<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, Box<dyn Error>> {
    let opt_aggr_counts = Spi::connect(|client| {
        match client.select("SELECT id, name FROM schema ORDER BY id;", None, None) {
            Ok(table) => Some(
                table
                    .filter_map(|row| {
                        match (row["id"].value::<Uuid>(), row["name"].value::<String>()) {
                            (Ok(Some(id)), Ok(Some(name))) => Some((id, name)),
                            _ => None,
                        }
                    })
                    .map(|schema| {
                        client
                            .select(
                                &format!("SELECT * FROM char_aggr_{}", schema.1),
                                Some(0),
                                None,
                            )
                            .map(|row| (schema.0, row.columns().unwrap() as i64))
                            .unwrap()
                    })
                    .collect::<Vec<(Uuid, i64)>>(),
            ),
            _ => None,
        }
    });
    let opt_field_counts = Spi::connect(|client| {
        match client.select(
            "SELECT schema_id, COUNT(*) FROM schema_field GROUP BY schema_id ORDER BY schema_id;",
            None,
            None,
        ) {
            Ok(table) => Some(
                table
                    .filter_map(|row| {
                        match (
                            row["schema_id"].value::<Uuid>(),
                            row["count"].value::<i64>(),
                        ) {
                            (Ok(Some(schema_id)), Ok(Some(count))) => Some((schema_id, count)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<(Uuid, i64)>>(),
            ),
            _ => None,
        }
    });
    if let (Some(aggr_counts), Some(field_counts)) = (opt_aggr_counts, opt_field_counts) {
        zip(aggr_counts, field_counts)
            .filter_map(|(aggr, field)| match aggr.1 - 1 != field.1 {
                true => Some(aggr.0),
                false => None,
            })
            .for_each(|schema| {
                Spi::run_with_args(
                    "SELECT refresh_char_aggr($1)",
                    Some(vec![(PgBuiltInOids::UUIDOID.oid(), schema.into_datum())]),
                )
                .unwrap();
            });
    }

    Ok(trigger.new())
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
    fn test_select_schema_name() -> Result<(), pgrx::spi::Error> {
        assert_eq!(
            "foo".to_string(),
            crate::select_schema_name(Uuid::from_bytes(SCHEMA_ID))?.unwrap()
        );
        Ok(())
    }

    #[pg_test]
    fn test_select_schema_field_cols() {
        assert_eq!(
            "charisma bigint, constitution bigint, dexterity bigint, intelligence bigint, strength bigint, wisdom bigint".to_string(),
            crate::select_schema_field_cols(Uuid::from_bytes(SCHEMA_ID))
        )
    }

    #[pg_test]
    fn test_refresh_char_aggr() -> Result<(), pgrx::spi::Error> {
        Spi::run("CREATE EXTENSION tablefunc;")?;
        crate::refresh_char_aggr(Uuid::from_bytes(SCHEMA_ID))?;
        assert_eq!(
            "Braugnor Quickcleaver".to_string(),
            Spi::get_one::<String>("SELECT name FROM char_aggr_foo;")?.unwrap()
        );
        Ok(())
    }

    #[pg_test]
    fn test_refresh_char_aggr_trigger() -> Result<(), pgrx::spi::Error> {
        Spi::run("CREATE EXTENSION tablefunc;")?;
        crate::refresh_char_aggr(Uuid::from_bytes(SCHEMA_ID))?;
        Spi::run("CREATE TRIGGER test_trigger AFTER INSERT OR UPDATE OR DELETE ON schema_field FOR EACH STATEMENT EXECUTE PROCEDURE refresh_char_aggr_trigger();")?;
        Spi::run("INSERT INTO schema_field (schema_id, fun_type, path) VALUES ('312c5ac5-23aa-4568-9d10-5949650bc8c0', 'int', 'bar');")?;
        assert_eq!(None, Spi::get_one::<i64>("SELECT bar FROM char_aggr_foo;")?);
        Ok(())
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
