use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use pgrx::prelude::*;

#[pg_extern(immutable)]
fn uuid7_time(uuid: pgrx::Uuid) -> Result<pgrx::Timestamp, &'static str> {
    if uuid[6] >> 4 == 7 {
        let ts = &uuid[0..6];
        let ms = (ts[0] as i64) << 40
            | (ts[1] as i64) << 32
            | (ts[2] as i64) << 24
            | (ts[3] as i64) << 16
            | (ts[4] as i64) << 8
            | (ts[5] as i64);

        if let Some(date) = NaiveDateTime::from_timestamp_millis(ms) {
            let time = date.time();

            Ok(pgrx::Timestamp::new_unchecked(
                date.year() as isize,
                date.month() as u8,
                date.day() as u8,
                time.hour() as u8,
                time.minute() as u8,
                f64::from(time.nanosecond()) / 1_000_000_000.0,
            ))
        } else {
            Err("Cannot convert timestamp to date.")
        }
    } else {
        Err("Wrong UUID version.")
    }
}

#[pg_extern]
fn gen_rand_uuid7() -> Result<pgrx::Uuid, String> {
    pgrx::Uuid::from_slice(uuid7::uuid7().as_bytes())
}

#[pg_extern]
fn gen_uuid7(ts: pgrx::Timestamp) -> Result<pgrx::Uuid, String> {
    let (h, m, s, ms) = ts.to_hms_micro();
    if let Some(datetime) = NaiveDate::from_ymd_opt(ts.year(), u32::from(ts.month()), u32::from(ts.day()))
        .and_then(|d| d.and_hms_micro_opt(u32::from(h), u32::from(m), u32::from(s), ms))
    {
        pgrx::Uuid::from_slice(
            uuid7::V7Generator::new(rand::rngs::OsRng)
                .generate_or_reset_core(datetime.timestamp_millis() as u64, 10_000)
                .as_bytes(),
        )
    } else {
        Err("Cannot convert timestamp to date.".to_string())
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    const UUID: pgrx::UuidBytes = [
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x69, 0xf0, 0x3e, 0xb0, 0x7d,
    ];

    #[pg_test]
    fn test_uuid7_to_time() {
        assert_eq!(
            Ok(Some(pgrx::Timestamp::new_unchecked(2023, 5, 29, 10, 36, 0.724))),
            Spi::get_one_with_args::<pgrx::Timestamp>(
                "SELECT uuid7_time($1);",
                vec![(PgBuiltInOids::UUIDOID.oid(), pgrx::Uuid::from_bytes(UUID).into_datum(),)],
            )
        );
    }

    #[pg_test]
    fn test_gen_uuid7() {
        let ts = pgrx::Timestamp::new_unchecked(2000, 1, 1, 0, 0, 0.0);
        let uuid = Spi::get_one_with_args::<pgrx::Uuid>(
            "SELECT gen_uuid7($1);",
            vec![(PgBuiltInOids::TIMESTAMPOID.oid(), ts.into_datum())],
        )
        .ok()
        .flatten()
        .unwrap();
        assert_eq!(
            Ok(Some(ts)),
            Spi::get_one_with_args::<pgrx::Timestamp>(
                "SELECT uuid7_time($1);",
                vec![(PgBuiltInOids::UUIDOID.oid(), uuid.into_datum(),)],
            )
        );
    }
}
