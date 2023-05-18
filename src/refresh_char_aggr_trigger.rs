use std::iter::zip;

use pgrx::{prelude::*, Uuid};

fn aggr_counts() -> Result<Vec<(Uuid, i64)>, pgrx::spi::Error> {
    Spi::connect(|client| {
        client
            .select("SELECT id, name FROM schema ORDER BY id;", None, None)
            .map(|table| {
                table
                    .filter_map(|row| {
                        row["id"]
                            .value::<Uuid>()
                            .ok()
                            .flatten()
                            .zip(row["name"].value::<String>().ok().flatten())
                    })
                    .map(|(schema_id, schema_name)| {
                        client
                            .select(
                                &format!("SELECT * FROM char_aggr_{schema_name};"),
                                Some(0),
                                None,
                            )
                            .map(|row| row.columns().map(|cols| (schema_id, cols as i64)))
                            .and_then(|x| x)
                    })
                    .collect::<Result<Vec<(Uuid, i64)>, pgrx::spi::Error>>()
            })
            .and_then(|res| res)
    })
}

fn field_counts() -> Result<Vec<(Uuid, i64)>, pgrx::spi::Error> {
    Spi::connect(|client| {
        client
            .select(
                "SELECT schema_id, COUNT(*) FROM schema_field GROUP BY schema_id ORDER BY schema_id;",
                None,
                None,
            )
            .map(|table| {
                table
                    .filter_map(|row| {
                        row["schema_id"]
                            .value::<Uuid>()
                            .ok()
                            .flatten()
                            .zip(row["count"].value::<i64>().ok().flatten())
                    })
                    .collect::<Vec<(Uuid, i64)>>()
            })
    })
}

#[pg_trigger]
pub fn refresh_char_aggr_trigger<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, pgrx::spi::Error> {
    if let (Ok(aggr_counts), Ok(field_counts)) = (aggr_counts(), field_counts()) {
        zip(aggr_counts, field_counts)
            .filter_map(|((aggr_id, aggr_count), (_, field_count))| {
                match aggr_count - 1 != field_count {
                    true => Spi::run_with_args(
                        "SELECT refresh_char_aggr($1);",
                        Some(vec![(PgBuiltInOids::UUIDOID.oid(), aggr_id.into_datum())]),
                    )
                    .ok(),
                    false => None,
                }
            })
            .for_each(drop);
    }

    Ok(trigger.new())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_char_aggr_sync() -> Result<(), pgrx::spi::Error> {
        Spi::run("CREATE EXTENSION tablefunc;")?;
        Spi::run("SELECT refresh_char_aggr('312c5ac5-23aa-4568-9d10-5949650bc8c0')")?;
        Spi::run("CREATE TRIGGER test_trigger AFTER INSERT OR UPDATE OR DELETE ON schema FOR EACH ROW EXECUTE PROCEDURE char_aggr_sync();")?;
        Spi::run("INSERT INTO schema (name) VALUES ('bar');")?;
        Spi::run("SELECT * FROM char_aggr_bar;")?;
        Ok(())
    }
}
