DROP TABLE "public"."rule_num";
DROP TABLE "public"."trait";
DROP TABLE "public"."sub_skill";
DROP TABLE "public"."skill";
DROP TABLE "public"."game";

DELETE FROM "public"."_migration" WHERE "name" = '230625095922_game';
NOTIFY pgrst, 'reload schema';
