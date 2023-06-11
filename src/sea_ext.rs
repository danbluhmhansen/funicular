use crate::sea_vals_to_args;
use pgrx::prelude::*;

pub trait SeaRunExt {
    fn run(&self) -> Result<(), pgrx::spi::Error>;
}

pub trait SeaGetOneExt {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error>;
}

impl SeaRunExt for sea_query::SelectStatement {
    fn run(&self) -> Result<(), pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::run_with_args(&query, Some(sea_vals_to_args(values)))
        } else {
            Spi::run(&query)
        }
    }
}

impl SeaGetOneExt for sea_query::SelectStatement {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::get_one_with_args(&query, sea_vals_to_args(values))
        } else {
            Spi::get_one(&query)
        }
    }
}

impl SeaRunExt for sea_query::InsertStatement {
    fn run(&self) -> Result<(), pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::run_with_args(&query, Some(sea_vals_to_args(values)))
        } else {
            Spi::run(&query)
        }
    }
}

impl SeaGetOneExt for sea_query::InsertStatement {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::get_one_with_args(&query, sea_vals_to_args(values))
        } else {
            Spi::get_one(&query)
        }
    }
}

impl SeaRunExt for sea_query::UpdateStatement {
    fn run(&self) -> Result<(), pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::run_with_args(&query, Some(sea_vals_to_args(values)))
        } else {
            Spi::run(&query)
        }
    }
}

impl SeaGetOneExt for sea_query::UpdateStatement {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::get_one_with_args(&query, sea_vals_to_args(values))
        } else {
            Spi::get_one(&query)
        }
    }
}

impl SeaRunExt for sea_query::DeleteStatement {
    fn run(&self) -> Result<(), pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::run_with_args(&query, Some(sea_vals_to_args(values)))
        } else {
            Spi::run(&query)
        }
    }
}

impl SeaGetOneExt for sea_query::DeleteStatement {
    fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        let (query, values) = self.build(sea_query::PostgresQueryBuilder);
        if values.iter().count() > 0 {
            Spi::get_one_with_args(&query, sea_vals_to_args(values))
        } else {
            Spi::get_one(&query)
        }
    }
}

impl SeaRunExt for sea_query::TableCreateStatement {
    fn run(&self) -> Result<(), pgrx::spi::Error> {
        Spi::run(&self.build(sea_query::PostgresQueryBuilder))
    }
}

impl SeaRunExt for sea_query::TableDropStatement {
    fn run(&self) -> Result<(), pgrx::spi::Error> {
        Spi::run(&self.build(sea_query::PostgresQueryBuilder))
    }
}
