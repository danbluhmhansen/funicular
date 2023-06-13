use pgrx::prelude::*;

pub trait IntoPgrxArg {
    fn into_arg(self) -> (pgrx::PgOid, Option<pg_sys::Datum>);
}

impl IntoPgrxArg for pgrx::Uuid {
    fn into_arg(self) -> (pgrx::PgOid, Option<pg_sys::Datum>) {
        (PgBuiltInOids::UUIDOID.oid(), self.into_datum())
    }
}
