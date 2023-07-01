#!/usr/bin/env nu

def main [name: string] {
  let now = (date now | date format "%y%m%d%H%M%S")
  let migration = $"($now)_($name)"
  let pascal = $"($now)(($name) | str pascal-case)"

  let line = (open db/src/migrations.rs | lines | enumerate | each { |l| if ($l.item | str length) == 0 { $l.index } } |
    skip 3 | take 1).0
  open db/src/migrations.rs | lines | insert $line $"pg_migration!\(_($migration)\);" | str join "\n" |
    save --force db/src/migrations.rs

  let up = (open db/src/migrations.rs | lines | enumerate |
    each { |l| if $l.item =~ '(SELECT _[0-9]*\w*_up)' { $l.index } } | reverse).0 + 1
  open db/src/migrations.rs | lines | insert $up $"    Spi::run\(\"SELECT _($migration)_up\(\);\"\)?;" | str join "\n" |
    save --force db/src/migrations.rs

  let down = (open db/src/migrations.rs | lines | enumerate |
    each { |l| if $l.item =~ '(SELECT _[0-9]*\w*_down)' { $l.index } }).0
  open db/src/migrations.rs | lines | insert $down $"    Spi::run\(\"SELECT _($migration)_down\(\);\"\)?;" | str join "\n" |
    save --force db/src/migrations.rs

  let path = $"db/src/migrations/_($migration)"
  mkdir $path

  $"INSERT INTO \"public\".\"_migration\" VALUES \('($migration)'\);
NOTIFY pgrst, 'reload schema';\n" | save $"($path)/up.sql"
  $"DELETE FROM \"public\".\"_migration\" WHERE \"name\" = '($migration)';
NOTIFY pgrst, 'reload schema';\n" | save $"($path)/down.sql"
}
