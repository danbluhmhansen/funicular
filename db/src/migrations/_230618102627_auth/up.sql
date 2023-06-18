CREATE ROLE "anon" NOLOGIN;

GRANT USAGE ON SCHEMA "public" TO "anon";

GRANT SELECT ON "public"."character"               TO "anon";
GRANT SELECT ON "public"."character_numeric_field" TO "anon";
GRANT SELECT ON "public"."character_trait"         TO "anon";
GRANT SELECT ON "public"."field"                   TO "anon";
GRANT SELECT ON "public"."numeric_rule"            TO "anon";
GRANT SELECT ON "public"."schema"                  TO "anon";
GRANT SELECT ON "public"."text_rule"               TO "anon";
GRANT SELECT ON "public"."trait"                   TO "anon";

INSERT INTO "_migration" VALUES ('230618102627_auth');
