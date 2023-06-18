#!/usr/bin/env nu

def main [name: string] {
  let now = (date now | date format "%y%m%d%H%M%S")
  let migration = $"($now)_($name)"
  let pascal = $"($now)(($name) | str pascal-case)"

  let line = (open db/src/migrations.rs | lines | enumerate | each { |l| if ($l.item | str length) == 0 { $l.index } } |
    skip 1 | take 1).0
  open db/src/migrations.rs | lines | insert $line $"mod _($migration);" | str join "\n" |
    save --force db/src/migrations.rs

  let path = $"db/src/migrations/_($migration)"
  mkdir $path

  $"use crate::migrations::Migration;
  use pgrx::prelude::*;

  struct _($pascal);

  impl Migration for _($pascal) {
      fn up\(\) -> Result<\(\), spi::Error> {
          if !Spi::get_one_with_args::<bool>\(
              r#\"SELECT EXISTS \(SELECT 1 FROM \"_migration\" WHERE \"name\" = $1 LIMIT 1\);\"#,
              vec![\(
                  PgBuiltInOids::TEXTOID.oid\(\),
                  \"($migration)\".into_datum\(\),
              \)],
          \)
          .is_ok_and\(|o| !o.is_some_and\(|b| !b\)\)
          {
              Spi::run\(include_str!\(\"up.sql\"\)\)?;
          }
          Ok\(\(\)\)
      }

      fn down\(\) -> Result<\(\), spi::Error> {
          if Spi::get_one_with_args::<bool>\(
              r#\"SELECT EXISTS \(SELECT 1 FROM \"_migration\" WHERE \"name\" = $1 LIMIT 1\);\"#,
              vec![\(
                  PgBuiltInOids::TEXTOID.oid\(\),
                  \"($migration)\".into_datum\(\),
              \)],
          \)
          .is_ok_and\(|o| o.is_some_and\(|b| b\)\)
          {
              Spi::run\(include_str!\(\"down.sql\"\)\)?;
          }
          Ok\(\(\)\)
      }
  }

  #[pg_extern]
  pub fn _($migration)_up\(\) -> Result<\(\), spi::Error> {
      _($pascal)::up\(\)
  }

  #[pg_extern]
  pub fn _($migration)_down\(\) -> Result<\(\), spi::Error> {
      _($pascal)::down\(\)
  }" | save $"($path)/mod.rs"

  $"INSERT INTO \"_migration\" VALUES \('($migration)'\);" | save $"($path)/up.sql"
  $"DELETE FROM \"_migration\" WHERE \"name\" = '($migration)';" | save $"($path)/down.sql"
}
