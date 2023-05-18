use std::error::Error;
use std::iter::zip;

use pgrx::prelude::*;
use pgrx::Uuid;

#[pg_trigger]
pub(crate) fn refresh_char_aggr_trigger<'a>(
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
                                &format!("SELECT * FROM char_aggr_{};", schema.1),
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
            .for_each(|schema_id| {
                Spi::run_with_args(
                    "SELECT refresh_char_aggr($1);",
                    Some(vec![(PgBuiltInOids::UUIDOID.oid(), schema_id.into_datum())]),
                )
                .unwrap();
            });
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
