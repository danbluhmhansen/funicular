default:
  @just --list

app:
  cd app && trunk serve

add migration name:
  nu tools/migrations/add.nu {{name}}

rm migration:
  nu tools/migrations/rm.nu

sqlc command:
  psql postgresql://postgres:postgres@localhost:5432/funicular --csv --command """{{command}}"""

sqlf file:
  psql postgresql://postgres:postgres@localhost:5432/funicular --csv --file """{{file}}"""
