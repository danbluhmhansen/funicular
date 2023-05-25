use crate::{
    fun_type::FunType,
    models::{Schema, SchemaField},
    sea_select::SeaSelect,
    sea_select_ext::SeaSelectExt,
    spi_heap_tuple_data_ext::SpiHeapTupleDataExt,
};

use pgrx::{prelude::*, Uuid};

/// Select a schema's name by its ID.
fn select_schema_name(schema_id: Uuid) -> Result<Option<String>, pgrx::spi::Error> {
    sea_query::Query::select()
        .from(Schema::Table)
        .column(Schema::Name)
        .and_where(
            sea_query::Expr::col(Schema::Id).eq(uuid::Uuid::from_bytes(*schema_id.as_bytes())),
        )
        .get_one()
}

/// Maps a schema's fields to a string like `name text, {path1} {fun_type1}, {path2} {fun_type2}`.
fn select_schema_field_cols(schema_id: Uuid) -> String {
    match Spi::connect(|client| -> Result<Vec<String>, pgrx::spi::Error> {
        client
            .sea_select(
                sea_query::Query::select()
                    .from(SchemaField::Table)
                    .columns([SchemaField::Path, SchemaField::FunType])
                    .and_where(
                        sea_query::Expr::col(SchemaField::SchemaId)
                            .eq(uuid::Uuid::from_bytes(*schema_id.as_bytes())),
                    )
                    .order_by(SchemaField::Path, sea_query::Order::Asc),
            )?
            .map(|row| match row.two::<String, FunType>() {
                Ok(Some((path, fun_type))) if fun_type == FunType::Int4 => {
                    Ok(format!("{path} bigint"))
                }
                Err(e) => Err(e),
                _ => Ok("".to_string()),
            })
            .collect()
    }) {
        Ok(mut cols) => {
            cols.insert(0, "name text".to_string());
            cols.join(", ")
        }
        _ => "name text".to_string(),
    }
}

/// (Re-)Creates a schema's character aggregate view.
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
                                WHEN schema_field.fun_type = ''Int4'' THEN effect.val::int
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
            Some("foo".to_string()),
            crate::refresh_char_aggr::select_schema_name(Uuid::from_bytes(SCHEMA_ID))?
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
            Some("Braugnor Quickcleaver".to_string()),
            Spi::get_one::<String>("SELECT name FROM char_aggr_foo;")?
        );
        Ok(())
    }
}
