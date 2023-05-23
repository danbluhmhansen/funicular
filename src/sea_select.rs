use super::sea_vals_to_args;

use pgrx::{prelude::*, spi::SpiClient};

pub trait SeaSelect {
    fn sea_select(
        &self,
        select: &mut sea_query::SelectStatement,
    ) -> Result<spi::SpiTupleTable, spi::Error>;
}

impl SeaSelect for SpiClient<'_> {
    fn sea_select(
        &self,
        select: &mut sea_query::SelectStatement,
    ) -> Result<spi::SpiTupleTable, spi::Error> {
        let (query, values) = select.build(sea_query::PostgresQueryBuilder);
        self.select(&query, None, Some(sea_vals_to_args(values)))
    }
}
