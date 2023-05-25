use super::sea_vals_to_args;

use pgrx::prelude::*;

pub trait SeaSelectExt {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error>;
    fn run(&self) -> Result<(), pgrx::spi::Error>;
}

impl SeaSelectExt for sea_query::SelectStatement {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::get_one_with_args(&query, sea_vals_to_args(values))
        } else {
            Spi::get_one(&query)
        }
    }

    fn run(&self) -> Result<(), pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::run_with_args(&query, Some(sea_vals_to_args(values)))
        } else {
            Spi::run(&query)
        }
    }
}
