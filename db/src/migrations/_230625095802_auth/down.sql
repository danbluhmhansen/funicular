DROP OWNED BY "anon";
DROP ROLE "anon";

DELETE FROM "public"."_migration" WHERE "name" = '230625095802_auth';
NOTIFY pgrst, 'reload schema';
