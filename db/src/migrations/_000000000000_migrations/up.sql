CREATE TABLE "_migration" ("name" text PRIMARY KEY);
COMMENT ON TABLE "_migration" IS 'Track which migrations are applied to the database.';
INSERT INTO "_migration" VALUES ('000000000000_migrations');
