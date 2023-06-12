use pgrx::{prelude::*, spi::SpiClient};

fn sea_vals_to_args(values: sea_query::Values) -> Vec<(PgOid, Option<pg_sys::Datum>)> {
    values
        .into_iter()
        .map(|v| match v {
            sea_query::Value::Bool(Some(b)) => (PgBuiltInOids::BOOLOID.oid(), b.into_datum()),
            sea_query::Value::TinyInt(Some(i)) => (PgBuiltInOids::INT2OID.oid(), i.into_datum()),
            sea_query::Value::SmallInt(Some(i)) => (PgBuiltInOids::INT2OID.oid(), i.into_datum()),
            sea_query::Value::Int(Some(i)) => (PgBuiltInOids::INT4OID.oid(), i.into_datum()),
            sea_query::Value::BigInt(Some(i)) => (PgBuiltInOids::INT8OID.oid(), i.into_datum()),
            sea_query::Value::Float(Some(f)) => (PgBuiltInOids::FLOAT4OID.oid(), f.into_datum()),
            sea_query::Value::Double(Some(d)) => (PgBuiltInOids::FLOAT8OID.oid(), d.into_datum()),
            sea_query::Value::String(Some(s)) => (PgBuiltInOids::TEXTOID.oid(), s.into_datum()),
            sea_query::Value::Char(Some(c)) => (PgBuiltInOids::CHAROID.oid(), c.into_datum()),
            sea_query::Value::Bytes(Some(b)) => (PgBuiltInOids::BYTEAOID.oid(), b.into_datum()),
            sea_query::Value::Uuid(Some(u)) => (
                PgBuiltInOids::UUIDOID.oid(),
                pgrx::Uuid::from_bytes(u.into_bytes()).into_datum(),
            ),
            _ => todo!(),
        })
        .collect()
}

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
