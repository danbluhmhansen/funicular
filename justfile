alias t := test

default:
  @just --list

test crate='':
  cargo test {{ if crate == 'app' { '--bin funicular_app' } else if crate == 'db' { '--lib db' } else { '' } }}

add migration name:
  nu tools/migrations/add.nu {{name}}

rm migration:
  nu tools/migrations/rm.nu
