#!/usr/bin/env nu

let last = (ls db/src/migrations/ | get name | last)
rm -r $last

let name = ($last | str substring 18..)
open db/src/migrations.rs | lines | each { |l| if not ($l | str contains $"pg_migration!\(($name)\);") { $l } } |
  str join "\n" | save --force db/src/migrations.rs
