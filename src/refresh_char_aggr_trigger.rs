use std::iter::zip;

use pgrx::{prelude::*, Uuid};

use crate::{
    models::{RefreshCharAggr, Schema, SchemaField},
    sea_ext::SeaRunExt,
    sea_select::SeaSelect,
    spi_heap_tuple_data_ext::SpiHeapTupleDataExt,
};

/// Count character aggregate fields.
fn aggr_counts() -> Result<Vec<(Uuid, i64)>, pgrx::spi::Error> {
    Spi::connect(|client| {
        client
            .sea_select(
                sea_query::Query::select()
                    .from(Schema::Table)
                    .columns([Schema::Id, Schema::Name])
                    .order_by(Schema::Id, sea_query::Order::Asc),
            )
            .map(|table| {
                table
                    .filter_map(|row| row.two::<Uuid, String>().ok().flatten())
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

/// Count schema fields.
fn field_counts() -> Result<Vec<(Uuid, i64)>, pgrx::spi::Error> {
    Spi::connect(|client| {
        client
            .sea_select(
                sea_query::Query::select()
                    .from(SchemaField::Table)
                    .column(SchemaField::SchemaId)
                    .expr(sea_query::Expr::count(sea_query::Expr::asterisk()))
                    .group_by_col(SchemaField::SchemaId)
                    .order_by(SchemaField::SchemaId, sea_query::Order::Asc),
            )
            .map(|table| {
                table
                    .filter_map(|row| row.two::<Uuid, i64>().ok().flatten())
                    .collect::<Vec<(Uuid, i64)>>()
            })
    })
}

/// Trigger on `INSERT`, `UPDATE`, and `DELETE` for `schema_fields`.
/// When triggered, it will look for character aggregate views no longer matching their schema fields,
/// and sync them using [crate::refresh_char_aggr::refresh_char_aggr].
#[pg_trigger]
pub fn refresh_char_aggr_trigger<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, pgrx::spi::Error> {
    if let (Ok(aggr_counts), Ok(field_counts)) = (aggr_counts(), field_counts()) {
        zip(aggr_counts, field_counts)
            .filter_map(|((aggr_id, aggr_count), (_, field_count))| {
                match aggr_count - 1 != field_count {
                    true => Some(
                        sea_query::Query::select()
                            .expr(
                                sea_query::Func::cust(RefreshCharAggr)
                                    .arg(uuid::Uuid::from_bytes(*aggr_id.as_bytes())),
                            )
                            .run(),
                    ),

                    false => None,
                }
            })
            .for_each(drop);
    }

    match trigger.op() {
        Ok(PgTriggerOperation::Insert) | Ok(PgTriggerOperation::Update) => Ok(trigger.new()),
        _ => Ok(trigger.old()),
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_refresh_char_aggr_trigger() -> Result<(), pgrx::spi::Error> {
        Spi::run("SELECT migrations_up();")?;
        Spi::run("SELECT fun_seed();")?;
        Spi::run("CREATE EXTENSION tablefunc;")?;
        Spi::run("SELECT refresh_char_aggr('01886715-04a4-7a8a-9c1d-ba69f03eb07d')")?;
        Spi::run("CREATE TRIGGER test_trigger AFTER INSERT OR UPDATE OR DELETE ON schema_field FOR EACH STATEMENT EXECUTE PROCEDURE refresh_char_aggr_trigger();")?;
        Spi::run("INSERT INTO schema_field (schema_id, fun_type, path) VALUES ('01886715-04a4-7a8a-9c1d-ba69f03eb07d', 'Int4', 'bar');")?;
        assert_eq!(None, Spi::get_one::<i64>("SELECT bar FROM char_aggr_foo;")?);
        Ok(())
    }
}
