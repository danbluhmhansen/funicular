use std::error::Error;

use pgrx::{prelude::*, Uuid};

#[pg_trigger]
pub fn char_aggr_sync<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, Box<dyn Error>> {
    match trigger.op().unwrap() {
        PgTriggerOperation::Insert => {
            if let Some(new) = trigger.new() {
                if let Some(schema_id) = new.get_by_name::<Uuid>("id")? {
                    Spi::run_with_args(
                        "SELECT refresh_char_aggr($1);",
                        Some(vec![(PgBuiltInOids::UUIDOID.oid(), schema_id.into_datum())]),
                    )?;
                }
            }
            Ok(trigger.new())
        }
        PgTriggerOperation::Update => {
            if let Some(new) = trigger.new() {
                if let Some(schema_name) = new.get_by_name::<String>("name")? {
                    Spi::run(&format!(
                        "DROP MATERIALIZED VIEW IF EXISTS char_aggr_{schema_name};",
                    ))?;
                }
                if let Some(schema_id) = new.get_by_name::<Uuid>("id")? {
                    Spi::run_with_args(
                        "SELECT refresh_char_aggr($1);",
                        Some(vec![(PgBuiltInOids::UUIDOID.oid(), schema_id.into_datum())]),
                    )?;
                }
            }
            Ok(trigger.new())
        }
        PgTriggerOperation::Delete => {
            if let Some(old) = trigger.old() {
                if let Some(schema_name) = old.get_by_name::<String>("name")? {
                    Spi::run(&format!(
                        "DROP MATERIALIZED VIEW IF EXISTS char_aggr_{schema_name};",
                    ))?;
                }
            }
            Ok(trigger.old())
        }
        PgTriggerOperation::Truncate => todo!(),
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_refresh_char_aggr_trigger() -> Result<(), pgrx::spi::Error> {
        Spi::run("CREATE EXTENSION tablefunc;")?;
        Spi::run("SELECT refresh_char_aggr('312c5ac5-23aa-4568-9d10-5949650bc8c0')")?;
        Spi::run("CREATE TRIGGER test_trigger AFTER INSERT OR UPDATE OR DELETE ON schema_field FOR EACH STATEMENT EXECUTE PROCEDURE refresh_char_aggr_trigger();")?;
        Spi::run("INSERT INTO schema_field (schema_id, fun_type, path) VALUES ('312c5ac5-23aa-4568-9d10-5949650bc8c0', 'int', 'bar');")?;
        assert_eq!(None, Spi::get_one::<i64>("SELECT bar FROM char_aggr_foo;")?);
        Ok(())
    }
}
