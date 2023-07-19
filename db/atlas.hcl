env "local" {
  src = "file://schema.hcl"
  url = "postgres://postgres:postgres@localhost:5432/funicular?sslmode=disable"
  dev = "postgres://postgres:postgres@localhost:5432/dev?sslmode=disable"
}
