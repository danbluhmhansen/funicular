alias t := test

default:
  @just --list

test crate='':
  cargo test {{ if crate == 'app' { '--bin funicular_app' } else if crate == 'db' { '--lib db' } else { '' } }}

add migration name:
  nu tools/migrations/add.nu {{name}}

rm migration:
  nu tools/migrations/rm.nu

sqlc command:
  psql postgresql://postgres:postgres@localhost:5432/funicular --csv --command """{{command}}"""

sqlf file:
  psql postgresql://postgres:postgres@localhost:5432/funicular --csv --file """{{file}}"""

app gen:
  #!/usr/bin/env nu
  cd app/api
  openapi-generator-cli generate --config config.yml --generator-name typescript --input-spec http://localhost:3000
