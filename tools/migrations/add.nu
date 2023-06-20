#!/usr/bin/env nu

def main [name: string] {
  let now = (date now | date format "%y%m%d%H%M%S")
  let migration = $"($now)_($name)"
  let pascal = $"($now)(($name) | str pascal-case)"

  let line = (open db/src/migrations.rs | lines | enumerate | each { |l| if ($l.item | str length) == 0 { $l.index } } |
    skip 3 | take 1).0
  open db/src/migrations.rs | lines | insert $line $"pg_migration!\(_($migration)\);" | str join "\n" |
    save --force db/src/migrations.rs

  let path = $"db/src/migrations/_($migration)"
  mkdir $path

  $"INSERT INTO \"_migration\" VALUES \('($migration)'\);" | save $"($path)/up.sql"
  $"DELETE FROM \"_migration\" WHERE \"name\" = '($migration)';" | save $"($path)/down.sql"
}
