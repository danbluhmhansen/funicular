use pgrx::prelude::*;

#[pg_extern(immutable)]
fn base58_encode_uuid(uuid: pgrx::Uuid) -> String {
    bs58::encode(uuid.as_bytes()).into_string()
}

#[pg_extern(immutable)]
fn base58_decode_uuid(text: String) -> Result<pgrx::Uuid, String> {
    match bs58::decode(text).into_vec() {
        Ok(bytes) => pgrx::Uuid::from_slice(&bytes),
        _ => Err("Failed to decode text".to_string()),
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    const UUID: pgrx::UuidBytes = [
        0x01, 0x88, 0x67, 0x15, 0x04, 0xa4, 0x7a, 0x8a, 0x9c, 0x1d, 0xba, 0x69, 0xf0, 0x3e, 0xb0, 0x7d,
    ];
    const BASE58: &str = "ByjhnmR7FHPTBrVpX91a4";

    #[pg_test]
    fn test_base58_encode_uuid() {
        assert_eq!(
            Ok(Some(BASE58.to_string())),
            Spi::get_one_with_args::<String>(
                "SELECT base58_encode_uuid($1);",
                vec![(PgBuiltInOids::UUIDOID.oid(), pgrx::Uuid::from_bytes(UUID).into_datum())],
            )
        );
    }

    #[pg_test]
    fn text_base58_decode_uuid() {
        assert_eq!(
            Ok(Some(pgrx::Uuid::from_bytes(UUID))),
            Spi::get_one_with_args(
                "SELECT base58_decode_uuid($1)",
                vec![(PgBuiltInOids::TEXTOID.oid(), BASE58.into_datum())]
            )
        )
    }
}
