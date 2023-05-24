use pgrx::{prelude::*, spi::SpiHeapTupleData};

pub trait SpiHeapTupleDataExt {
    fn one<A: IntoDatum + FromDatum>(&self) -> Result<Option<A>, pgrx::spi::Error>;

    fn two<A: IntoDatum + FromDatum, B: IntoDatum + FromDatum>(
        &self,
    ) -> Result<Option<(A, B)>, pgrx::spi::Error>;

    fn three<A: IntoDatum + FromDatum, B: IntoDatum + FromDatum, C: IntoDatum + FromDatum>(
        &self,
    ) -> Result<Option<(A, B, C)>, pgrx::spi::Error>;
}

impl SpiHeapTupleDataExt for SpiHeapTupleData {
    fn one<A: IntoDatum + FromDatum>(&self) -> Result<Option<A>, pgrx::spi::Error> {
        self[1].value()
    }

    fn two<A: IntoDatum + FromDatum, B: IntoDatum + FromDatum>(
        &self,
    ) -> Result<Option<(A, B)>, pgrx::spi::Error> {
        Ok(self[1].value::<A>()?.zip(self[2].value::<B>()?))
    }

    fn three<A: IntoDatum + FromDatum, B: IntoDatum + FromDatum, C: IntoDatum + FromDatum>(
        &self,
    ) -> Result<Option<(A, B, C)>, pgrx::spi::Error> {
        match (
            self[1].value::<A>()?,
            self[2].value::<B>()?,
            self[3].value::<C>()?,
        ) {
            (Some(a), Some(b), Some(c)) => Ok(Some((a, b, c))),
            _ => Ok(None),
        }
    }
}
