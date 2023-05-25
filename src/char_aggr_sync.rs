use std::error::Error;

use pgrx::{prelude::*, Uuid};

use crate::{models::RefreshCharAggr, sea_select_ext::SeaSelectExt};

/// Creates an aggregate view over characters, for a schema, using [crate::refresh_char_aggr::refresh_char_aggr].
fn create_view(schema_id: Uuid) -> Result<(), pgrx::spi::Error> {
    sea_query::Query::select()
        .expr(
            sea_query::Func::cust(RefreshCharAggr)
                .arg(uuid::Uuid::from_bytes(*schema_id.as_bytes())),
        )
        .run()
}

/// Drops a schema's character aggregate view.
fn drop_view(schema_name: String) -> Result<(), pgrx::spi::Error> {
    Spi::run(&format!(
        "DROP MATERIALIZED VIEW IF EXISTS char_aggr_{schema_name};",
    ))
}

/// Trigger to run on `INSERT`, `UPDATE`, and `DELETE` operations on `schemas`.
/// When triggered, it will create and/or drop the schema's character aggregate view.
#[pg_trigger]
pub fn char_aggr_sync<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, Box<dyn Error>> {
    match trigger.op() {
        Ok(PgTriggerOperation::Insert) => {
            if let Some(Some(schema_id)) = trigger.new().and_then(|new| new.get_by_name("id").ok())
            {
                create_view(schema_id)?;
            }
            Ok(trigger.new())
        }
        Ok(PgTriggerOperation::Update) => {
            if let (Some((Ok(Some(new_name)), Ok(Some(schema_id)))), Some(Some(old_name))) = (
                trigger
                    .new()
                    .map(|new| (new.get_by_name::<String>("name"), new.get_by_name("id"))),
                trigger
                    .old()
                    .and_then(|old| old.get_by_name::<String>("name").ok()),
            ) {
                if new_name != old_name {
                    drop_view(old_name)?;
                    create_view(schema_id)?;
                }
            }
            Ok(trigger.new())
        }
        Ok(PgTriggerOperation::Delete) => {
            if let Some(Some(schema_name)) =
                trigger.old().and_then(|old| old.get_by_name("name").ok())
            {
                drop_view(schema_name)?;
            }
            Ok(trigger.old())
        }
        Ok(PgTriggerOperation::Truncate) => todo!(),
        Err(e) => Err(Box::new(e)),
    }
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
