set positional-arguments

alias a := app
alias t := test

default:
  @just --list

app *args='':
  cd app && deno task "$@"

test:
  cargo test

add migration name:
  nu tools/migrations/add.nu {{name}}

rm migration:
  nu tools/migrations/rm.nu

sqlc command:
  pgcli postgresql://postgres:postgres@localhost:5432/funicular --csv --command """{{command}}"""

sqlf file:
  pgcli postgresql://postgres:postgres@localhost:5432/funicular --csv --file """{{file}}"""
