CREATE ROLE "anon";

GRANT USAGE ON SCHEMA "public" TO "anon";

INSERT INTO "public"."_migration" VALUES ('230625095802_auth');
NOTIFY pgrst, 'reload schema';
