DROP OWNED BY "anon";
DROP ROLE "anon";

DELETE FROM "_migration" WHERE "name" = '230625095802_auth';
NOTIFY pgrst, 'reload schema';
