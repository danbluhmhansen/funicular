use pgrx::prelude::*;

#[pg_extern(immutable)]
fn slugify(input: String) -> Result<String, regex::Error> {
    regex::Regex::new("[^a-z0-9 ]").map(|r| {
        r.replace_all(&input.to_lowercase(), "")
            .to_string()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
    })
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn simple() {
        assert_eq!(
            Ok(Some("foo".to_string())),
            Spi::get_one_with_args::<String>(
                "SELECT slugify($1);",
                vec![(PgBuiltInOids::TEXTOID.oid(), "Foo".into_datum())],
            )
        );
    }

    #[pg_test]
    fn whitespace() {
        assert_eq!(
            Ok(Some("foo-bar".to_string())),
            Spi::get_one_with_args::<String>(
                "SELECT slugify($1);",
                vec![(PgBuiltInOids::TEXTOID.oid(), "Foo Bar".into_datum())],
            )
        );
    }

    #[pg_test]
    fn symbols() {
        assert_eq!(
            Ok(Some("foo-bar".to_string())),
            Spi::get_one_with_args::<String>(
                "SELECT slugify($1);",
                vec![(PgBuiltInOids::TEXTOID.oid(), "Foo - Bar!".into_datum())],
            )
        );
    }
}
