env "local" {
  src = "file://schema.hcl"
  url = "postgres://localhost:28815/funicular?sslmode=disable"
  dev = "postgres://localhost:28815/postgres?sslmode=disable"
}
